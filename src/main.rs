use btleplug::api::{Central, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::{Manager, Peripheral as Device};
use futures::stream::StreamExt;
use std::error::Error;
use std::time::{Duration, Instant};
use tokio::time;

const INDOOR_BIKE_DATA_UUID: uuid::Uuid = uuid::Uuid::from_u128(0x00002ad2_0000_1000_8000_00805f9b34fb);

struct WorkoutState {
    last_update: Instant,
    total_distance_meters: f64,
    current_speed: f64,
    current_power: f64,
    current_rpm: f64,
    elapsed_time_bike: u16,
}

impl WorkoutState {
    fn new() -> Self {
        Self {
            last_update: Instant::now(),
            total_distance_meters: 0.0,
            current_speed: 0.0,
            current_power: 0.0,
            current_rpm: 0.0,
            elapsed_time_bike: 0,
        }
    }

    fn update_distance(&mut self) {
        let now = Instant::now();
        let duration = now.duration_since(self.last_update).as_secs_f64();
        // Distância acumulada: (Velocidade em m/s) * tempo decorrido
        self.total_distance_meters += (self.current_speed / 3.6) * duration;
        self.last_update = now;
    }

    fn display(&self) {
        // Limpa o console e reseta o cursor
        print!("\x1B[2J\x1B[1;1H");
        println!("=========================================");
        println!("       PAINEL GALLANT SPINNING (Rust)    ");
        println!("=========================================");
        println!("🚀 Velocidade:  {:.1} km/h", self.current_speed);
        println!("🔄 Cadência:    {:.0} RPM", self.current_rpm);
        println!("⚡ Potência:    {:.0} W", self.current_power);
        println!("📏 Distância:   {:.2} m", self.total_distance_meters);
        println!("⏱️  Tempo Bike:  {} s", self.elapsed_time_bike);
        println!("=========================================");
        println!(" Pressione Ctrl+C para encerrar o treino ");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let manager = Manager::new().await?;
    let adapters = manager.adapters().await?;
    let central = adapters.into_iter().next().ok_or("Adaptador não encontrado")?;

    println!("--- Buscando Dispositivo: GLT... ---");
    central.start_scan(ScanFilter::default()).await?;

    let mut gallant_bike: Option<Device> = None;

    for _ in 0..15 {
        let peripherals = central.peripherals().await?;
        for p in peripherals {
            if let Some(props) = p.properties().await? {
                let name = props.local_name.unwrap_or_default();
                if name.contains("GLT") || name.contains("Gallant") {
                    println!("\n✅ Bicicleta encontrada: {}", name);
                    gallant_bike = Some(p);
                    break;
                }
            }
        }
        if gallant_bike.is_some() { break; }
        time::sleep(Duration::from_secs(1)).await;
    }

    let bike = gallant_bike.ok_or("Bicicleta não encontrada. Pedale para ativar!")?;
    bike.connect().await?;
    bike.discover_services().await?;

    let chars = bike.characteristics();
    let data_char = chars.iter()
        .find(|c| c.uuid == INDOOR_BIKE_DATA_UUID)
        .ok_or("Serviço FTMS não disponível.")?;

    bike.subscribe(data_char).await?;
    let mut stream = bike.notifications().await?;

    let mut state = WorkoutState::new();

    while let Some(notification) = stream.next().await {
        let data = notification.value;

        match data.len() {
            18 => {
                // Pacote de Performance (Velocidade e Potência)
                let raw_speed = u16::from_le_bytes([data[2], data[3]]) as f64;
                // Fator de correção Gallant para km/h real (aprox 0.168 do bruto)
                state.current_speed = raw_speed * 0.168;

                let raw_power = i16::from_le_bytes([data[9], data[10]]) as f64;
                // Ajuste de escala para Potência (W)
                state.current_power = raw_power * 0.83;

                state.update_distance();
            }
            6 => {
                // Pacote de Cadência e Tempo
                let raw_cad = u16::from_le_bytes([data[2], data[3]]) as f64;
                // Conversão de pulsos magnéticos para RPM real
                state.current_rpm = raw_cad / 42.7;

                state.elapsed_time_bike = u16::from_le_bytes([data[4], data[5]]);
            }
            _ => continue,
        }

        state.display();
    }

    Ok(())
}
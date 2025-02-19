use ::log::debug;

fn main() -> anyhow::Result<()> {
    debug!("MAIN: carga dependencias");
    use esp_idf_svc::hal::adc::{AdcContConfig, AdcContDriver, AdcMeasurement, Attenuated};
    use esp_idf_svc::hal::peripherals::Peripherals;
    
    debug!("PATCHES");
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    debug!("INI: inicializa perifericos y config");
    let peripherals = Peripherals::take()?;
    let config = AdcContConfig::default();

    debug!("Configura el canal ADC");
    // Configurar el canal ADC
    let adc_1_channel_0 = Attenuated::db11(peripherals.pins.gpio34);

    debug!("Crea el driver ADC continuo");
    // Crear el driver ADC continuo (requiere I2S0 como segundo parámetro)
    let mut adc = AdcContDriver::new(
        peripherals.adc1,
        peripherals.i2s0, // <-- Añadir el periférico I2S0
        &config, // <-- Pasar como referencia
        adc_1_channel_0,
    )?;

    debug!("Inicia el ADC");
    adc.start()?;

    debug!("Crea el buffer de lecturas");
    // Buffer para muestras
    let mut samples = [AdcMeasurement::default(); 100];

    debug!("Entra en el loop");
    loop {
        // Intentar leer muestras del ADC
        match adc.read(&mut samples, 10) {
            Ok(num_read) => {
                debug!("Read {} measurements", num_read);
                for index in 0..num_read {
                    debug!("Value: {}", samples[index].data());
                }
            }
            Err(e) => {
                debug!("Error al leer el ADC: {:?}", e); // Mensaje de depuración en caso de error
            }
        }
    }
}
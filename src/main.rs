use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::{Gpio2, PinDriver};
use esp_idf_svc::sys::link_patches;
use esp_idf_svc::log::EspLogger;
use log::info;

fn main() {
    // Necesario para aplicar parches de enlace en el runtime
    link_patches();

    // Inicializa el logger para usar las facilidades de logging de ESP
    EspLogger::initialize_default();

    info!("Iniciando programa de parpadeo de LED");

    // Configura el pin GPIO2 como salida (conecta el LED a este pin)
    let mut led = PinDriver::output(unsafe { Gpio2::new() }).unwrap();

    loop {
        info!("Encendiendo el LED");
        led.set_high().unwrap(); // Enciende el LED
        FreeRtos::delay_ms(500); // Espera 500 ms

        info!("Apagando el LED");
        led.set_low().unwrap(); // Apaga el LED
        FreeRtos::delay_ms(500); // Espera 500 ms
    }
}
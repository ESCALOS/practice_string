use practice_string::daily_activities::{generate_daily_activities, model::{Activity, IncidentType, Priority, SoftwareType}};

fn main() {
    let activities = vec![
        Activity::new(1, SoftwareType::Web, IncidentType::Incident, "Balanza".to_string(), "Tarja".to_string(), "No dejaba tarjar el sistema".to_string(), Priority::Middle, "Desbloquear el contenedor".to_string(), String::from("Gerald")),
        Activity::new(2, SoftwareType::Apk, IncidentType::Requirement, "App".to_string(), "Login".to_string(), "No se puede iniciar sesión".to_string(), Priority::High, "Revisar el servidor de autenticación".to_string(), String::from("Alice")),
        Activity::new(3, SoftwareType::Web, IncidentType::Incident, "Software Contable".to_string(), "Reporte".to_string(), "Error al generar reporte".to_string(), Priority::Low, "Actualizar el software".to_string(), String::from("Bob")),
        Activity::new(4, SoftwareType::Web, IncidentType::Requirement, "Portal".to_string(), "Registro".to_string(), "Problemas al registrarse".to_string(), Priority::Middle, "Revisar el formulario de registro".to_string(), String::from("Charlie")),
        Activity::new(5, SoftwareType::Apk, IncidentType::Incident, "App".to_string(), "Notificaciones".to_string(), "No llegan notificaciones".to_string(), Priority::High, "Revisar el servicio de notificaciones".to_string(), String::from("Dave")),
        Activity::new(6, SoftwareType::Web, IncidentType::Requirement, "Editor de Texto".to_string(), "Guardado".to_string(), "No guarda los cambios".to_string(), Priority::Middle, "Revisar permisos de escritura".to_string(), String::from("Eve")),
        Activity::new(7, SoftwareType::Web, IncidentType::Incident, "E-commerce".to_string(), "Pago".to_string(), "Error al procesar pagos".to_string(), Priority::High, "Revisar la pasarela de pagos".to_string(), String::from("Frank")),
        Activity::new(8, SoftwareType::Apk, IncidentType::Requirement, "App".to_string(), "Actualización".to_string(), "No se puede actualizar".to_string(), Priority::Low, "Revisar la conexión a internet".to_string(), String::from("Grace")),
        Activity::new(9, SoftwareType::Web, IncidentType::Incident, "Software de Diseño".to_string(), "Exportar".to_string(), "Error al exportar archivos".to_string(), Priority::Middle, "Actualizar el software".to_string(), String::from("Hank")),
        Activity::new(10, SoftwareType::Web, IncidentType::Requirement, "Blog".to_string(), "Comentarios".to_string(), "No se pueden publicar comentarios".to_string(), Priority::Low, "Revisar el sistema de comentarios".to_string(), String::from("Ivy")),
        Activity::new(11, SoftwareType::Apk, IncidentType::Incident, "App".to_string(), "Cámara".to_string(), "No funciona la cámara".to_string(), Priority::High, "Revisar permisos de la cámara".to_string(), String::from("Jack"))
    ];

    let start_date = "2025-01-01";
    let end_date = "2025-01-05";

    let daily_activities = generate_daily_activities(activities, start_date, end_date, 3);

    println!("Software         | Incidente       | Problema     | Prioridad       | Solución       | Fecha     ");
    println!("-------------------------------------------------------------------------------------------------");

    for (date, activities) in daily_activities {
        for activity in activities {
            println!("{} | {} | {} | {} | {} | {}", 
                activity.get_software_type().label(),
                activity.get_incident_type().label(), 
                activity.get_problem(),
                activity.get_priority().label(),
                activity.get_solution(),
                date
            )
        }
    }
}
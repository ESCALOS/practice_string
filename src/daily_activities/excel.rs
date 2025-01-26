use calamine::{open_workbook, Reader, XlsError, Xlsx};
use chrono::NaiveDate;
use rust_xlsxwriter::{Format, Workbook};

use super::model::{Activity, IncidentType, Priority, SoftwareType};

fn parse_priority(priority: String) -> Priority {
    match priority.to_lowercase().as_str() {
        "alta" => Priority::High,
        "media" => Priority::Middle,
        _ => Priority::Low,
    }
}

fn parse_software(software: String) -> SoftwareType {
    match software.to_lowercase().as_str() {
        "software apk" => SoftwareType::Apk,
        _ => SoftwareType::Web
    }
}

fn parse_incident(incident: String) -> IncidentType {
    match incident.to_lowercase().as_str() {
        "incidente" => IncidentType::Incident,
        _ => IncidentType::Requirement
    }
}

pub fn load_activities_from_excel(file_path: &str) -> Vec<Activity> {
    let mut workbook: Xlsx<_> = open_workbook(file_path).expect("No se pudo abrir el excel");
    match workbook.worksheet_range("Hoja1") {
        Ok(range) => {
            let mut data: Vec<Activity> = Vec::new();

            for row in range.rows().skip(1) {

                data.push(Activity::new(1, 
                    parse_software(row[0].to_string()), 
                    parse_incident(row[1].to_string()), 
                    row[2].to_string(), 
                    row[3].to_string(), 
                    row[4].to_string(), 
                    parse_priority(row[5].to_string()), 
                    row[6].to_string(), 
                    String::from("Carlos")));
            }

            data
        },
        Err(err) => {
            println!("No se encontró la hoja 1 en el libro: {:?}", err);
            return Vec::new();
        }
    }
}

pub fn export_activities_to_excel(activities: &Vec<(NaiveDate, Vec<Activity>)>) -> Result<(), XlsError> {
    // Crear el workbook
    let mut workbook = Workbook::new();

    let sheet = workbook.add_worksheet();

    let bold_format = Format::new().set_bold();
    
    // Escribir encabezados
    let _ = sheet.write_with_format(0, 0, "Software",&bold_format);
    let _ = sheet.write_with_format(0, 1, "Incidente",&bold_format);
    let _ = sheet.write_with_format(0, 2, "Proceso",&bold_format);
    let _ = sheet.write_with_format(0, 3, "Subproceso",&bold_format);
    let _ = sheet.write_with_format(0, 4, "Problema",&bold_format);
    let _ = sheet.write_with_format(0, 5, "Prioridad",&bold_format);
    let _ = sheet.write_with_format(0, 6, "Solución",&bold_format);
    let _ = sheet.write_with_format(0, 7, "Encargado",&bold_format);
    let _ = sheet.write_with_format(0, 8, "Fecha",&bold_format);

    let mut row = 1;

    // Escribir los datos
    for (_, (date,activities)) in activities.iter().enumerate() {
        for activity in activities {
            let _ = sheet.write((row) as u32, 0, activity.get_software_type().label());
            let _ = sheet.write((row) as u32, 1, activity.get_incident_type().label());
            let _ = sheet.write((row) as u32, 2, activity.get_process());
            let _ = sheet.write((row) as u32, 3, activity.get_subprocess());
            let _ = sheet.write((row) as u32, 4, activity.get_problem());
            let _ = sheet.write((row) as u32, 5, activity.get_priority().label());
            let _ = sheet.write((row) as u32, 6, activity.get_solution());
            let _ = sheet.write((row) as u32, 7, String::from("Carlos"));
            let _ = sheet.write((row) as u32, 8, date.to_string());

            row += 1;
        }
    }

    // Guardar el archivo
    let _ = workbook.save("Hola.xlsx");

    Ok(())

}

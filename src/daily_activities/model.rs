#[derive(Debug,Clone)]
pub enum SoftwareType {
    Web,
    Apk
}

impl SoftwareType {
    pub fn label(&self) -> &str {
        match self {
            SoftwareType::Apk => "Sio Apk",
            SoftwareType::Web => "Sio Web"
        }
    }
}

#[derive(Debug,Clone)]
pub enum IncidentType {
    Incident,
    Requirement
}

impl IncidentType {
    pub fn label(&self) -> &str {
        match self {
            IncidentType::Incident => "Incidente",
            IncidentType::Requirement => "Requerimiento"
        }
    }
}

#[derive(Debug,Clone)]
pub enum Priority {
    Low,
    Middle,
    High
}

impl Priority {
    pub fn label(&self) -> &str {
        match self {
            Priority::Low => "Baja",
            Priority::Middle => "Media",
            Priority::High => "Alta"
        }
    }
}

#[derive(Debug,Clone)]
pub struct Activity {
    id: u8,
    software_type: SoftwareType,
    incident_type: IncidentType,
    process: String,
    subprocess: String,
    problem: String,
    priority: Priority,
    solution: String,
    responsable: String
}

impl Activity {
    pub fn new(
        id: u8,
        software_type: SoftwareType,
        incident_type: IncidentType,
        process: String,
        subprocess: String,
        problem: String,
        priority: Priority,
        solution: String,
        responsable: String
    ) -> Self {
        Activity {
            id,
            software_type,
            incident_type,
            process,
            subprocess,
            problem,
            priority,
            solution,
            responsable
        }
    }

    pub fn get_id(&self) ->u8 {
        self.id
    }

    pub fn set_id(&mut self, id: u8) {
        self.id = id;
    }

    pub fn get_software_type(&self) -> &SoftwareType {
        &self.software_type
    }

    pub fn set_software_type(&mut self, software_type: SoftwareType) {
        self.software_type = software_type;
    }

    pub fn get_incident_type(&self) -> &IncidentType {
        &self.incident_type
    }

    pub fn set_incident_type(&mut self, incident_type: IncidentType) {
        self.incident_type = incident_type;
    }

    pub fn get_process(&self) -> &str {
        &self.process
    }

    pub fn set_process(&mut self, process: String) {
        self.process = process;
    }

    pub fn get_subprocess(&self) -> &str {
        &self.subprocess
    }

    pub fn set_subprocess(&mut self, subprocess: String) {
        self.subprocess = subprocess;
    }

    pub fn get_problem(&self) -> &str {
        &self.problem
    }

    pub fn set_problem(&mut self, problem: String) {
        self.problem = problem;
    }

    pub fn get_priority(&self) -> &Priority {
        &self.priority
    }

    pub fn set_priority(&mut self, priority: Priority) {
        self.priority = priority;
    }

    pub fn get_solution(&self) -> &str {
        &self.solution
    }

    pub fn set_solution(&mut self, solution: String) {
        self.solution = solution;
    }

    pub fn get_responsable(&self) -> &str {
        &self.responsable
    }

    pub fn set_responsable(&mut self, responsable: String) {
        self.responsable = responsable;
    }
}

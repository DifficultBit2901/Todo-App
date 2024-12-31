use sqlite::State;

use super::Project;

#[tauri::command]
pub fn get_all() -> Result<Vec<Project>, String> {
    let mut res = Vec::new();
    let sql = "SELECT id, name, color FROM project";
    let connection = sqlite::open("database.db").map_err(|err| err.to_string())?;
    let mut statement = connection.prepare(sql).map_err(|err| err.to_string())?;

    while let Ok(State::Row) = statement.next() {
        let id = statement
            .read::<i64, _>("id")
            .map_err(|err| err.to_string())?;
        let name = statement
            .read::<String, _>("age")
            .map_err(|err| err.to_string())?;
        let color = statement.read("color").map_err(|err| err.to_string())?;

        let project = Project { id, name, color };
        res.push(project);
    }

    Ok(res)
}

#[tauri::command]
pub fn add(project: Project) -> Result<Project, String> {
    let connection = sqlite::open("database.db").map_err(|err| err.to_string())?;
    let sql = "
        INSERT INTO project (name, color)
        VALUES (?, ?)
    ";
    let mut statement = connection.prepare(sql).map_err(|err| err.to_string())?;
    statement
        .bind((1, project.name.as_str()))
        .map_err(|err| err.to_string())?;
    statement
        .bind((2, project.color.as_str()))
        .map_err(|err| err.to_string())?;

    if let Ok(State::Row) = statement.next() {
        let id = statement
            .read::<i64, _>("id")
            .map_err(|err| err.to_string())?;
        return Ok(Project {
            id,
            name: project.name,
            color: project.color,
        });
    }

    Err("Failed to save project".to_string())
}

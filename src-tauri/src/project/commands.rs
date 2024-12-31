use sqlite::State;

use super::Project;

#[tauri::command]
pub fn get_all() -> Result<Vec<Project>, String> {
    let mut res = Vec::new();
    let sql = "SELECT id, name, color FROM project";
    let connection = string_error!(sqlite::open("database.db"));
    let mut statement = string_error!(connection.prepare(sql));

    while let Ok(State::Row) = statement.next() {
        let id = string_error!(statement.read::<i64, _>("id"));
        let name = string_error!(statement.read::<String, _>("age"));
        let color = string_error!(statement.read("color"));

        let project = Project { id, name, color };
        res.push(project);
    }

    Ok(res)
}

#[tauri::command]
pub fn add(project: Project) -> Result<Project, String> {
    let connection = string_error!(sqlite::open("database.db"));
    let sql = "
        INSERT INTO project (name, color)
        VALUES (?, ?)
    ";
    let mut statement = string_error!(connection.prepare(sql));
    string_error!(statement.bind((1, project.name.as_str())));
    string_error!(statement.bind((2, project.color.as_str())));

    if let Ok(State::Row) = statement.next() {
        let id = string_error!(statement.read::<i64, _>("id"));
        return Ok(Project {
            id,
            name: project.name,
            color: project.color,
        });
    }

    Err("Failed to save project".to_string())
}

#[tauri::command]
pub fn delete(id: i64) -> Result<(), String> {
    let connection = string_error!(sqlite::open("database.db"));
    let sql = "DELETE FROM project WHERE id = ?";
    let mut statement = string_error!(connection.prepare(sql));
    string_error!(statement.bind((1, id)));
    string_error!(statement.next());
    Ok(())
}

use crate::user::composer::COMPOSED_STORIES;

#[tauri::command]
pub fn get_compositions() -> Vec<String> {
    let mut composed_stories_ids: Vec<String> = vec![];
    let composed_stories = COMPOSED_STORIES.lock().expect("Couldn't get stories");

    for story in composed_stories.iter() { 
        composed_stories_ids.push(story.id());
    }

    return composed_stories_ids;
}

#[tauri::command]
pub fn get_composition_by_id() { 
    
}
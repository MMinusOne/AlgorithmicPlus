// use crate::user::composer::COMPOSED_STORIES;

// #[tauri::command]
// pub fn get_compositions() -> Vec<String> {
    // let mut composed_stories_ids: Vec<String> = vec![];

    // for story in COMPOSED_STORIES { 
    //     let unlocked_story = story().lock().unwrap();
    //     composed_stories_ids.push(unlocked_story.id().into());
    // }

    // return composed_stories_ids;
// }

#[tauri::command]
pub fn get_composition_by_id() { 
    
}
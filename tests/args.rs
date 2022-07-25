pub struct Args {
    /// to create file

    pub target: Vec<String>,
    /// to delete

    pub del: Option<Vec<String>>,
    /// creates files inside target directory-first arguement is target

    pub r#in: Option<Vec<String>>,
    /// deletes files inside target directory-first arguement is target

    pub din: Option<Vec<String>>,

    /// send the file to trash can

    pub trash: Option<Vec<String>>,

    /// undoes the last create or trash

    pub undo: bool,
    
    /// shows command history

    pub show: bool,
    
    /// Renam a file

    pub ren: Option<Vec<String>>,

    /// Move a file from one directory to another.
    pub mve: Option<Vec<String>>
}
impl Default for Args {
    fn default() -> Args {
        Args {
            target: vec![
                "./test_dir/target_test.txt".to_string(),
                "./test_dir/target_test/".to_string(),
                ],
            del: Some(vec![
                "./test_dir/del_test.txt".to_string(),
                "./test_dir/del_test/".to_string(),
                ]),
            r#in: Some(vec![
                "./test_dir".to_string(),
                "r#in_test.txt".to_string(),
                "/r#in_test/".to_string(),
                ]),
            din: Some(vec![
                "./test_dir".to_string(),
                "din_test.txt".to_string(),
                "/din_test/".to_string(),
                ]),
            trash: Some(vec![
                "./test_dir/trash_test.txt".to_string(),
                "./test_dir/trash_test/".to_string(),
                ]),
            undo: true,
            show: true,
            ren: Some(vec![
                "./test_dir/rename_test.txt".to_string(),
                "./test_dir/rename_passed.txt".to_string(),
            ]),
            mve: Some(vec![
                "./test_dir/move_test.txt".to_string(),
                "./test_dir/move_test/".to_string(),
            ])
        }
    }
}
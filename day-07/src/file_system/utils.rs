pub fn compute_new_cwd_from(s: &str, cwd: &str) -> String {
    let parts: Vec<&str> = s.split(" ").collect();
    let target_dir = parts.get(2).unwrap().to_string();

    if target_dir == ".." {
        let mut cwd_parts: Vec<&str> = cwd.split("/").filter(|p| p != &"").collect();
        cwd_parts.pop();

        let mut new_cwd = String::from("/");
        new_cwd.push_str(&cwd_parts.join("/"));

        new_cwd
    } else {
        let mut fully_qualified_target_dir = cwd.clone().to_string();
        if target_dir != "/" {
            if cwd != "/" {
                fully_qualified_target_dir.push_str("/");
            }

            fully_qualified_target_dir.push_str(&target_dir);
        }

        println!("cd {}", fully_qualified_target_dir);
        fully_qualified_target_dir
    }
}

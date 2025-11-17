use bcrypt::{hash, DEFAULT_COST};

fn main() {
    let password = "123456";
    let hash = hash(password, DEFAULT_COST).unwrap();
    println!("BCrypt Hash for password '{}':", password);
    println!("{}", hash);
    println!();
    println!("Run this SQL to update admin user:");
    println!("UPDATE sys_user SET password='{}' WHERE username='admin';", hash);
}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod test {
    #[test]
    fn green_test() {
        assert!(true);
    }

    #[test]
    #[should_panic]
    fn should_panic() {
        assert!(false);
    }
}

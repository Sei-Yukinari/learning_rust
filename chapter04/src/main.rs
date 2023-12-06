fn main() {
    {
        let point = Box::new((0.625, 0.5)); // point allocated here
        let label = format!("{:?}", point); // label allocated here
        assert_eq!(label, "(0.625, 0.5)");
    }

    {
        struct Person {
            name: String,
            birth: i32,
        }
        let mut composers = Vec::new();
        composers.push(Person {
            name: "Palestrina".to_string(),
            birth: 1525,
        });
        composers.push(Person {
            name: "Dowland".to_string(),
            birth: 1563,
        });
        composers.push(Person {
            name: "Lully".to_string(),
            birth: 1632,
        });
        for composer in &composers {
            println!("{}, born {}", composer.name, composer.birth);
        }
    }
    {
        let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
        let t = &s;
        let u = &s;
    }

    {
        use std::rc::Rc;
        let s: Rc<String> = Rc::new("shirataki".to_string());
        let t: Rc<String> = s.clone();
        let u: Rc<String> = s.clone();

        assert!(s.contains("shira"));
        assert_eq!(t.find("taki"), Some(5));
        println!("{} are quite chewy, almost bouncy, but lack flavor", u);
    }
}

fn print_padovan() {
    let mut padovan = vec![1, 1, 1];
    for i in 3..10 {
        let next = padovan[i - 3] + padovan[i - 2];
        padovan.push(next);
    }
    println!("P(1..10) = {:?}", padovan);
}

#[cfg(test)]
mod tests {
    use crate::print_padovan;

    #[test]
    fn print_padovan_exec() {
        print_padovan()
    }
}
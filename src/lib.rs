mod rspark {

    #[allow(dead_code)]
    const TICKS: [char; 8] = [
        '\u{2581}', '\u{2582}', '\u{2583}', '\u{2584}', '\u{2585}', '\u{2586}', '\u{2587}',
        '\u{2588}',
    ];

    #[allow(dead_code)]
    pub fn render(v: &Vec<i32>) -> Result<String, &'static str> {
        if v.len() < 2 {
            return Err("Invalid vector parameter");
        }
        let max = v.iter().max().unwrap();
        let min = v.iter().min().unwrap();
        let scale = (max - min) as f32 / 7.;
        let mut s = String::new();
        for e in v.iter() {
            let i = (e - min) / scale as i32;
            s.push_str(&TICKS[i as usize].to_string());
        }
        Ok(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render() {
        let v = vec![2, 250, 670, 890, 2, 430, 11, 908, 123, 57];
        let res = rspark::render(&v).unwrap();
        println!("{}", res);
        assert_eq!("▁▂▆▇▁▄▁█▁▁", res);
    }

    #[test]
    fn test_render_err() {
        let v = vec![2];
        let res = rspark::render(&v);
        match res {
            Ok(_) => panic!("Error expected."),
            _ => (),
        }
    }
}

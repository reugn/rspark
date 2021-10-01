#![crate_name = "rspark"]

pub mod rspark {

    use std::error::Error;
    use std::fmt;

    /// The Unicode representation of the graph ticks.
    const TICKS: [char; 8] = [
        '\u{2581}', '\u{2582}', '\u{2583}', '\u{2584}', '\u{2585}', '\u{2586}', '\u{2587}',
        '\u{2588}',
    ];

    /// Renders a graph for the given numeric vector.
    ///
    /// # Arguments
    ///
    /// * `v` - The numeric vector to render a graph.
    ///
    /// # Examples
    ///
    /// ```
    /// let v = vec![2, 250, 670, 890, 2, 430, 11, 908, 123, 57];
    /// let res = rspark::rspark::render(&v).unwrap();
    /// assert_eq!("▁▂▆▇▁▄▁█▁▁", res);
    /// ```
    pub fn render(v: &Vec<i32>) -> Result<String, RenderError> {
        let mut s = String::new();
        render_to(v, &mut s)
            .map(|ok_val| ok_val.to_string())
            .map_err(|err_val| err_val)
    }

    /// Renders a graph and appends it to the given string.
    ///
    /// # Arguments
    ///
    /// * `v` - The numeric vector to render a graph.
    /// * `s` - The mutable String pointer to append the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// let v = vec![2, 250, 670, 890, 2, 430, 11, 908, 123, 57];
    /// let mut s = String::from(">");
    /// let res = rspark::rspark::render_to(&v, &mut s).unwrap();
    /// assert_eq!(">▁▂▆▇▁▄▁█▁▁", res);
    /// ```
    pub fn render_to<'a>(v: &Vec<i32>, s: &'a mut String) -> Result<&'a str, RenderError> {
        if v.len() < 2 {
            return Err(RenderError::InvalidVectorParameter);
        }
        let max = v.iter().max().unwrap();
        let min = v.iter().min().unwrap();
        let scale = (max - min) as f32 / 7.;
        for e in v.iter() {
            let i = (e - min) / scale as i32;
            (*s).push_str(&TICKS[i as usize].to_string());
        }
        Ok(&s[..])
    }

    #[derive(Debug)]
    pub enum RenderError {
        /// The invalid vector parameter error.
        InvalidVectorParameter,
    }

    impl fmt::Display for RenderError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                RenderError::InvalidVectorParameter => f.write_str("InvalidVectorParameter"),
            }
        }
    }

    impl Error for RenderError {
        fn description(&self) -> &str {
            match *self {
                RenderError::InvalidVectorParameter => "Invalid vector parameter",
            }
        }
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
    fn test_render_to() {
        let v = vec![2, 250, 670, 890, 2, 430, 11, 908, 123, 57];
        let mut s = String::from(">");
        let res = rspark::render_to(&v, &mut s).unwrap();
        println!("{}", res);
        assert_eq!(">▁▂▆▇▁▄▁█▁▁", res);
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

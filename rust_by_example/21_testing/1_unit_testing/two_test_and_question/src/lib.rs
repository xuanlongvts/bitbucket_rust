pub fn sqrt(n: f64) -> Result<f64, String> {
    if n > 0.0 {
        Ok(n.powf(0.5))
    } else {
        Err("negative floats don't have square roots".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt() -> Result<(), String> {
        let x = 4.0;
        assert_eq!(sqrt(x)?.powf(2.0), x);

        Ok(())
    }
}

use async_trait::async_trait;
use reqwest::Response;
use size::Size;

#[async_trait]
pub trait Load {
    async fn load(data: Response) -> Self;
}

pub trait Resource {}

pub trait Compute:Resource {
    fn avg(f64_array: &Vec<f64>) -> f64 {
        f64_array.into_iter().fold(0.0, |acc, x| acc + x)
    }

    fn percentage(num:&f64) -> String {
        let rounded = f64::trunc(num  * 100.0) / 100.0; //
        (rounded * 100.0).to_string() + "%"
    }

    fn size(num: &u64) -> String {
        let size = Size::from_bytes(num.to_owned());
        format!("{}", size)
    }
}
// implements Compute across all structs implementing Resource
impl<T> Compute for T where T: Resource {}

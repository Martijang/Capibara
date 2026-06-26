
pub mod request{
    use clap::ValueEnum;
    use reqwest::StatusCode;
    #[derive(Debug, Clone, Copy, ValueEnum)]
    pub enum Method{
        Get,
        Post,
    }


    #[derive(Debug)]
    pub struct RequestResult{
        pub status: StatusCode,
        pub body: String
    }
}

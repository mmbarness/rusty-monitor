use async_trait::async_trait;

#[async_trait]
pub trait Setup {
    async fn create_static_data() {
        
    }

    fn seed_monitors() {
        // load average across cores - cpu_load_average
        // loads at each core - cpu_loads
        // ul/dl rate - upload_download
        // hd storage - hd_storage
        // ram - ram
    }
}
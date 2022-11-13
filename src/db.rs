use arcstr::ArcStr;
use color_eyre::eyre::Result;
use lateinit::LateInit;
use singleton::Singleton;

#[derive(Singleton, Default)]
pub struct Db {
  id_db: LateInit<sled::Db>,
}
impl Db {
  pub fn init(&self) -> Result<()> {
    let options = sled::Config::default().cache_capacity(1024 * 1024);
    let id_db = options.path("db/wit-notify/wit-id").open()?;

    self.id_db.init(id_db);
    Ok(())
  }
  pub fn record(&self, id: &ArcStr) -> bool {
    let mut contains = self
      .id_db
      .contains_key(id.as_bytes())
      .unwrap_or(true);
    if !contains && self.id_db.insert(id.as_bytes(), &[]).is_err(){
      contains = true
    }
    !contains
  }
}

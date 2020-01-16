use serde::Deserialize;

const GUWEN_URL: &str = "https://www.caoxingyu.club/guwen";
// const SENTENCE_URL: &str = "https://www.caoxingyu.club/guwen/sentence";

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Article {
  pub id: String,
  pub title: String,
  pub writer: String,
  pub r#type: Vec<String>,
  pub content: String,
  pub remark: Option<String>,
  pub translation: Option<String>,
  pub shangxi: Option<String>,
  #[serde(rename(deserialize = "audioUrl"))]
  pub audio_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Sentence {
  pub _id: String,
  pub name: String,
  pub from: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ArticleResp {
  pub total: usize,
  pub pages: usize,
  pub page: usize,
  pub pagesize: usize,
  pub data: Vec<Article>,
  #[serde(skip)]
  pub method: Method,
}

#[derive(Debug, Clone)]
pub enum Method {
  Page,
  Writer(String),
  Dynasty(String),
}

impl Default for Method {
  fn default() -> Method {
    Method::Page
  }
}

impl ArticleResp {
  pub fn by_page(page: usize) -> Result<Self, Box<dyn std::error::Error>> {
    let url = format!("{}/selectall?page={}", GUWEN_URL, page);
    Ok(reqwest::blocking::get(&url)?.json::<Self>()?)
  }

  pub fn by_writer(page: usize, writer: &str) -> Result<Self, Box<dyn std::error::Error>> {
    let url = format!(
      "{}/selectbywriter?page={}&writer={}",
      GUWEN_URL, page, writer
    );
    let mut resp = reqwest::blocking::get(&url)?.json::<Self>()?;
    resp.method = Method::Writer(writer.to_string());
    Ok(resp)
  }

  pub fn by_dynasty(page: usize, dynasty: &String) -> Result<Self, Box<dyn std::error::Error>> {
    let url = format!(
      "{}/selectbydynasty?page={}&dynasty={}",
      GUWEN_URL, page, dynasty
    );
    let mut resp = reqwest::blocking::get(&url)?.json::<Self>()?;
    resp.method = Method::Dynasty(dynasty.clone());
    Ok(resp)
  }

  // pub fn query_by_id(id: &String) -> Result<Option<Article>, Box<dyn std::error::Error>> {
  //   let url = format!("{}/selectbyid?id={}", GUWEN_URL, id);
  //   Ok(reqwest::blocking::get(&url)?.json::<Option<Article>>()?)
  // }

  pub fn next_page(&self) -> Result<Self, Box<dyn std::error::Error>> {
    match &self.method {
      Method::Page => Self::by_page(self.page + 1),
      Method::Dynasty(dynasty) => Self::by_dynasty(self.page + 1, &dynasty),
      Method::Writer(writer) => Self::by_writer(self.page + 1, &writer),
    }
  }

  pub fn prev_page(&self) -> Result<Self, Box<dyn std::error::Error>> {
    let page = if self.page == 1 { 1 } else { self.page - 1 };
    match &self.method {
      Method::Page => Self::by_page(page),
      Method::Dynasty(dynasty) => Self::by_dynasty(page, &dynasty),
      Method::Writer(writer) => Self::by_writer(page, &writer),
    }
  }
}

mod tests {
  use super::ArticleResp;

  #[test]
  fn test_page_1() {
    match ArticleResp::by_page(1) {
      Ok(resp) => (assert!(resp.data.len() > 0)),
      Err(_) => panic!(),
    }
  }

  #[test]
  fn test_page_1000001() {
    match ArticleResp::by_page(1_000_001) {
      Ok(resp) => assert!(resp.data.is_empty()),
      Err(_) => panic!(),
    }
  }

  #[test]
  fn test_writer_libai_p1() {
    match ArticleResp::by_writer(1, &"李白".to_string()) {
      Ok(resp) => assert!(!resp.data.is_empty()),
      Err(_) => panic!(),
    }
  }

  #[test]
  fn test_writer_libai_p100001() {
    match ArticleResp::by_writer(100_001, &"李白".to_string()) {
      Ok(resp) => assert_eq!(resp.data.len(), 0),
      Err(_) => panic!(),
    }
  }

  #[test]
  fn test_writer_unknown_p1() {
    match ArticleResp::by_writer(1, &"向东".to_string()) {
      Ok(resp) => assert_eq!(resp.data.len(), 0),
      Err(_) => panic!(),
    }
  }

  #[test]
  fn test_dynasty_tang_p1() {
    match ArticleResp::by_dynasty(1, &"唐代".to_string()) {
      Ok(resp) => assert_ne!(resp.data.len(), 0),
      Err(_) => panic!(),
    }
  }

  #[test]
  fn test_dynasty_tang_p100001() {
    match ArticleResp::by_dynasty(100001, &"唐代".to_string()) {
      Ok(resp) => assert_eq!(resp.data.len(), 0),
      Err(_) => panic!(),
    }
  }

  #[test]
  fn test_dynasty_fake_p1() {
    match ArticleResp::by_dynasty(100001, &"fake".to_string()) {
      Ok(resp) => assert_eq!(resp.data.len(), 0),
      Err(_) => panic!(),
    }
  }

  // #[test]
  // fn test_id_exit() {
  //   match ArticleResp::query_by_id(&"5b9a0136367d5c96f4cd2952".to_string()) {
  //     Ok(resp) => match resp {
  //       None => panic!(),
  //       Some(_) => (),
  //     },
  //     Err(_) => panic!(),
  //   }
  // }

  // #[test]
  // fn test_id_notexit() {
  //   if let Ok(_) = ArticleResp::query_by_id(&"fake12345678".to_string()) {
  //     panic!()
  //   }
  // }
}

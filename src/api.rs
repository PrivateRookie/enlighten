use serde::Deserialize;
use thiserror::Error;

const GUWEN_URL: &str = "https://www.caoxingyu.club/guwen";
const SENTENCE_URL: &str = "https://www.caoxingyu.club/guwen/sentence";
const WRITER_URL: &str = "https://www.caoxingyu.club/guwen/writer";

#[derive(Error, Debug)]
pub enum APIError {
    #[error("can not find object")]
    NotFound,
    #[error("invalid page should be >= 1")]
    InvalidPage,
}

#[derive(Debug, Clone)]
pub enum Method {
    Page,
    Writer(String),
    Dynasty(String),
    Keyword(String),
}

impl Default for Method {
    fn default() -> Method {
        Method::Page
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct ArticleSimple {
    pub id: String,
    pub title: String,
}

impl ArticleSimple {
    pub fn show(&self) -> Result<Article, Box<dyn std::error::Error>> {
        let url = format!("{}/selectbyid?id={}", GUWEN_URL, self.id);
        Ok(reqwest::blocking::get(&url)?.json::<Article>()?)
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Article {
    pub id: String,
    pub title: String,
    pub writer: String,
    // pub r#type: Vec<String>,
    pub content: String,
    pub remark: Option<String>,
    pub translation: Option<String>,
    pub shangxi: Option<String>,
    #[serde(rename(deserialize = "audioUrl"))]
    pub audio_url: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ArtListResp {
    pub total: usize,
    pub pages: usize,
    pub page: usize,
    #[serde(rename(deserialize = "pagesize"))]
    pub page_size: usize,
    #[serde(skip)]
    pub method: Method,
    pub data: Vec<ArticleSimple>,
}

impl ArtListResp {
    pub fn list_by_page(page: usize) -> Result<Self, Box<dyn std::error::Error>> {
        let url = format!("{}/selectall?page={}", GUWEN_URL, page);
        Ok(reqwest::blocking::get(&url)?.json::<Self>()?)
    }

    pub fn list_by_writer(page: usize, writer: String) -> Result<Self, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/selectbywriter?page={}&writer={}",
            GUWEN_URL, page, &writer
        );
        let mut resp = reqwest::blocking::get(&url)?.json::<Self>()?;
        resp.method = Method::Writer(writer);
        Ok(resp)
    }

    pub fn list_by_dynasty(
        page: usize,
        dynasty: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/selectbydynasty?page={}&dynasty={}",
            GUWEN_URL, page, &dynasty
        );
        let mut resp = reqwest::blocking::get(&url)?.json::<Self>()?;
        resp.method = Method::Dynasty(dynasty);
        Ok(resp)
    }

    pub fn list_by_keyword(
        page: usize,
        keyword: String,
    ) -> Result<ArtListResp, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/selectbykeyword?page={}&keyword={}",
            GUWEN_URL, page, &keyword
        );
        let mut resp = reqwest::blocking::get(&url)?.json::<ArtListResp>()?;
        resp.method = Method::Keyword(keyword);
        Ok(resp)
    }

    pub fn show(art_id: &str) -> Result<Article, Box<dyn std::error::Error>> {
        let url = format!("{}/selectbyid?id={}", GUWEN_URL, art_id);
        Ok(reqwest::blocking::get(&url)?.json::<Article>()?)
    }

    pub fn prev_page(&self) -> Result<ArtListResp, Box<dyn std::error::Error>> {
        let page = if self.page == 1 { 1 } else { self.page - 1 };
        match &self.method {
            Method::Page => Self::list_by_page(page),
            Method::Dynasty(dynasty) => Self::list_by_dynasty(page, dynasty.clone()),
            Method::Writer(writer) => Self::list_by_writer(page, writer.clone()),
            Method::Keyword(keyword) => Self::list_by_keyword(page, keyword.clone()),
        }
    }

    pub fn next_page(&self) -> Result<ArtListResp, Box<dyn std::error::Error>> {
        let page = self.page + 1;
        match &self.method {
            Method::Page => Self::list_by_page(page),
            Method::Dynasty(dynasty) => Self::list_by_dynasty(page, dynasty.clone()),
            Method::Writer(writer) => Self::list_by_writer(page, writer.clone()),
            Method::Keyword(keyword) => Self::list_by_keyword(page, keyword.clone()),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Sentence {
    pub id: String,
    pub name: String,
    pub from: String,
}

impl Sentence {
    pub fn retrive_orign(&self) -> Result<Article, Box<dyn std::error::Error>> {
        match ArtListResp::list_by_keyword(1, self.name.trim().to_owned()) {
            Ok(resp) => match resp.data.first() {
                None => Err(Box::new(APIError::NotFound)),
                Some(art) => art.show(),
            },
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct SentenceListResp {
    pub total: usize,
    pub pages: usize,
    pub page: usize,
    #[serde(rename(deserialize = "pagesize"))]
    pub page_size: usize,
    pub data: Vec<Sentence>,
}

impl SentenceListResp {
    pub fn list(page: usize) -> Result<Self, Box<dyn std::error::Error>> {
        let url = format!("{}/selectall?page={}", SENTENCE_URL, page);
        Ok(reqwest::blocking::get(&url)?.json::<Self>()?)
    }

    pub fn prev_page(&self) -> Result<Self, Box<dyn std::error::Error>> {
        let page = if self.page == 1 { 1 } else { self.page - 1 };
        Self::list(page)
    }

    pub fn next_page(&self) -> Result<Self, Box<dyn std::error::Error>> {
        Self::list(self.page + 1)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Writer {
    pub id: String,
    pub name: String,
    #[serde(rename(deserialize = "headImageUrl"), default = "String::new")]
    pub head_img_url: String,
    #[serde(rename(deserialize = "simpleIntro"), default = "String::new")]
    pub simple_intro: String,
    #[serde(rename(deserialize = "detailIntro"), default = "String::new")]
    pub detail_intro: String,
}

impl Writer {
    pub fn detail(&self) -> Result<Self, Box<dyn std::error::Error>> {
        let url = format!("{}/selectbyid?id={}", WRITER_URL, self.id);
        Ok(reqwest::blocking::get(&url)?.json::<Self>()?)
    }

    pub fn get_articles(&self, page: usize) -> Result<ArtListResp, Box<dyn std::error::Error>> {
        ArtListResp::list_by_writer(page, self.name.clone())
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct WriterListResp {
    pub total: usize,
    pub pages: usize,
    pub page: usize,
    #[serde(rename(deserialize = "pagesize"))]
    pub page_size: usize,
    pub data: Vec<Writer>,
}

impl WriterListResp {
    pub fn list(page: usize) -> Result<Self, Box<dyn std::error::Error>> {
        let url = format!("{}/selectall?page={}", WRITER_URL, page);
        Ok(reqwest::blocking::get(&url)?.json::<Self>()?)
    }

    pub fn prev_page(&self) -> Result<Self, Box<dyn std::error::Error>> {
        let page = if self.page == 1 { 1 } else { self.page - 1 };
        Self::list(page)
    }
    pub fn next_page(&self) -> Result<Self, Box<dyn std::error::Error>> {
        Self::list(self.page + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_show_art_exit() {
        let art = ArticleSimple {
            id: "5b9a0136367d5c96f4cd2952".to_owned(),
            title: "将进酒".to_owned(),
        };
        match art.show() {
            Err(_) => panic!(),
            _ => (),
        }
    }

    #[test]
    fn test_show_art_nonexit() {
        let art = ArticleSimple {
            id: "fake".to_owned(),
            title: "将进酒".to_owned(),
        };
        match art.show() {
            Ok(_) => panic!(),
            _ => (),
        }
    }

    #[test]
    fn test_art_list_page_1() {
        match ArtListResp::list_by_page(1) {
            Ok(resp) => assert_ne!(resp.data.len(), 0),
            Err(_) => panic!(),
        }
    }

    #[test]
    fn test_art_list_page_200000() {
        match ArtListResp::list_by_page(200000) {
            Ok(resp) => assert_eq!(resp.data.len(), 0),
            Err(_) => panic!(),
        }
    }

    #[test]
    fn test_art_list_writer_exit_p1() {
        match ArtListResp::list_by_writer(1, "李白".to_string()) {
            Ok(resp) => assert_ne!(resp.data.len(), 0),
            Err(_) => panic!(),
        }
    }

    #[test]
    fn test_art_list_writer_exit_p1000() {
        match ArtListResp::list_by_writer(1000, "李白".to_string()) {
            Ok(resp) => assert_eq!(resp.data.len(), 0),
            Err(_) => panic!(),
        }
    }

    #[test]
    fn test_art_list_writer_nonexit_p1() {
        match ArtListResp::list_by_writer(1, "向东".to_string()) {
            Ok(resp) => assert_eq!(resp.data.len(), 0),
            Err(_) => panic!(),
        }
    }

    #[test]
    fn test_art_list_dynasty_exit_p1() {
        match ArtListResp::list_by_dynasty(1, "唐代".to_string()) {
            Ok(resp) => assert_ne!(resp.data.len(), 0),
            Err(_) => panic!(),
        }
    }

    #[test]
    fn test_art_list_dynasty_exit_p20000() {
        match ArtListResp::list_by_dynasty(20000, "唐代".to_string()) {
            Ok(resp) => assert_eq!(resp.data.len(), 0),
            Err(_) => panic!(),
        }
    }

    #[test]
    fn test_art_list_dynasty_nonexit_p1() {
        match ArtListResp::list_by_dynasty(1, "燕朝".to_string()) {
            Ok(resp) => assert_eq!(resp.data.len(), 0),
            Err(_) => panic!(),
        }
    }

    #[test]
    fn test_art_list_keyword_p1() {
        match ArtListResp::list_by_keyword(1, "李白".to_string()) {
            Ok(resp) => assert_ne!(resp.data.len(), 0),
            Err(_) => panic!(),
        }
    }

    #[test]
    fn test_art_list_keyword_p20000() {
        match ArtListResp::list_by_keyword(20000, "李白".to_string()) {
            Ok(resp) => assert_eq!(resp.data.len(), 0),
            Err(_) => panic!(),
        }
    }

    #[test]
    fn test_sentence_retrive_origin_exit() {
        let sentence = Sentence {
            id: "5b9b713f367d5c55cca9c92a".to_string(),
            name: "山有木兮木有枝，心悦君兮君不知。".to_string(),
            from: "佚名《越人歌》".to_string(),
        };
        match sentence.retrive_orign() {
            Ok(art) => assert_eq!(art.id, "5b9a1448367d5cab186686a4".to_string()),
            Err(_) => panic!(),
        }
    }

    #[test]
    fn test_sentence_retrive_origin_nonexit() {
        let sentence = Sentence {
            id: "5b9b713f367d5c55cca9c92a".to_string(),
            name: "山有木兮木有枝，心悦君兮君不知。TDFS".to_string(),
            from: "佚名《越人歌》".to_string(),
        };
        match sentence.retrive_orign() {
            Ok(_) => panic!(),
            Err(_) => (),
        }
    }

    #[test]
    fn test_sentence_list_p1() {
        match SentenceListResp::list(1) {
            Ok(resp) => assert_ne!(resp.data.len(), 0),
            Err(_) => panic!(),
        }
    }

    #[test]
    fn test_writer_detail() {
        let writer = Writer {
            id: "5b9b6211367d5c24d8bcdc01".to_string(),
            name: "李白".to_string(),
            detail_intro: String::default(),
            simple_intro: String::default(),
            head_img_url: String::default(),
        };
        match writer.detail() {
            Err(_) => panic!(),
            _ => (),
        }
    }

    #[test]
    fn test_writer_articles() {
        let writer = Writer {
            id: "5b9b6211367d5c24d8bcdc01".to_string(),
            name: "李白".to_string(),
            detail_intro: String::default(),
            simple_intro: String::default(),
            head_img_url: String::default(),
        };
        match writer.get_articles(1) {
            Ok(resp) => assert_ne!(resp.data.len(), 0),
            Err(_) => panic!(),
        }
    }

    #[test]
    fn test_writer_list() {
        match WriterListResp::list(1) {
            Ok(resp) => assert_ne!(resp.data.len(), 0),
            Err(_) => panic!(),
        }
    }
}

use crate::application::port::driven::tag_repository;


struct TagRepository {}

impl tag_repository::TagRepository for TagRepository {
    fn find_one(&self, id: i32) -> Result<crate::domain::tag::Tag, crate::application::port::driven::errors::RepoSelectError> {
        todo!()
    }

    fn find_all(&self, username: &String, from: i64, to: i64) -> Result<Vec<crate::domain::tag::Tag>, crate::application::port::driven::errors::RepoFindAllError> {
        todo!()
    }

    fn create(&self, Tag: crate::domain::tag::Tag) -> Result<crate::domain::tag::Tag, crate::application::port::driven::errors::RepoCreateError> {
        todo!()
    }

    fn update(&self, Tag: crate::domain::tag::Tag) -> Result<crate::domain::tag::Tag, crate::application::port::driven::errors::RepoUpdateError> {
        todo!()
    }

    fn delete(&self, id: i32) -> Result<(), crate::application::port::driven::errors::RepoDeleteError> {
        todo!()
    }
}
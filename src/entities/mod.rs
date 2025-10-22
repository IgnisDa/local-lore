pub mod dependency;
pub mod project_dependency;

pub mod prelude {
    pub use super::dependency::Entity as Dependency;
    pub use super::project_dependency::Entity as ProjectDependency;
}

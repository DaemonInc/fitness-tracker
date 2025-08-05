use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ExercisesGetResponse {
    /// A list of exercises
    Status200_AListOfExercises
    (Vec<models::Exercise>)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum WorkoutsCreatePostResponse {
    /// Workout created successfully
    Status201_WorkoutCreatedSuccessfully
    ,
    /// Bad request, invalid input data
    Status400_BadRequest
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum WorkoutsGetResponse {
    /// A list of workouts
    Status200_AListOfWorkouts
    (Vec<models::Workout>)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum WorkoutsUuidGetResponse {
    /// A workout object
    Status200_AWorkoutObject
    (models::Workout)
    ,
    /// Workout not found
    Status404_WorkoutNotFound
}


/// Default
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Default<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// Get all exercises.
    ///
    /// ExercisesGet - GET /exercises
    async fn exercises_get(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
    ) -> Result<ExercisesGetResponse, E>;

    /// Create workout.
    ///
    /// WorkoutsCreatePost - POST /workouts/create
    async fn workouts_create_post(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
            body: &models::CreateWorkout,
    ) -> Result<WorkoutsCreatePostResponse, E>;

    /// Get all workouts.
    ///
    /// WorkoutsGet - GET /workouts
    async fn workouts_get(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
    ) -> Result<WorkoutsGetResponse, E>;

    /// Get workout by UUID.
    ///
    /// WorkoutsUuidGet - GET /workouts/{uuid}
    async fn workouts_uuid_get(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::WorkoutsUuidGetPathParams,
    ) -> Result<WorkoutsUuidGetResponse, E>;
}

use crate::types::{EndpointRet, PasswordErrors, ServerError};

use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use base64::{engine::general_purpose, Engine};
use chrono::{DateTime, Datelike, Utc};
use image::{io::Reader as ImageRader, Rgb};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::PgPool;
use std::{collections::HashMap, sync::Mutex, time::Instant};
use tinytemplate::TinyTemplate;
use ulid::Ulid;
use uuid::Uuid;


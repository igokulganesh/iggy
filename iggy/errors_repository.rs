use convert_case::{Case, Casing};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorRepositoryEntry {
    pub snake_case_name: String,
    pub code: u32,
    pub template: String,
    pub signature: String,
    pub converts_from: String,
}

// Adds computed fields to avoid re-computation in later usage
pub struct PreprocessedErrorRepositoryEntry {
    pub snake_case_name: String,
    pub code: u32,
    pub template: String,
    pub signature: String,
    pub converts_from: String,
    pub pascal_case_name: String,
    pub signature_wildcard_pattern: String,
}

fn get_full_signature_string(entry: &ErrorRepositoryEntry) -> String {
    match (entry.converts_from.is_empty(), entry.signature.is_empty()) {
        (true, true) => String::new(),
        (true, false) => format!("({})", entry.signature),
        (false, true) => format!("(#[from] {})", entry.converts_from),
        (false, false) => format!("({}, {})", entry.converts_from, entry.signature),
    }
}

fn to_wildcard_pattern(s: &str) -> String {
    if s.is_empty() {
        String::new()
    } else {
        let n = s.chars().filter(|&c| c == ',').count();
        format!("({})", vec!["_"; n + 1].join(", "))
    }
}

impl From<ErrorRepositoryEntry> for PreprocessedErrorRepositoryEntry {
    fn from(error_code: ErrorRepositoryEntry) -> Self {
        PreprocessedErrorRepositoryEntry {
            snake_case_name: error_code.snake_case_name.clone(),
            code: error_code.code,
            template: error_code.template.clone(),
            signature: error_code.signature.clone(),
            converts_from: error_code.converts_from.clone(),
            pascal_case_name: error_code.snake_case_name.to_case(Case::Pascal),
            signature_wildcard_pattern: {
                to_wildcard_pattern(&get_full_signature_string(&error_code))
            },
        }
    }
}

pub fn load_errors() -> Vec<ErrorRepositoryEntry> {
    let error_codes: Vec<ErrorRepositoryEntry> = vec![
        ErrorRepositoryEntry {
            snake_case_name: "error".to_string(),
            code: 1,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Error".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "invalid_configuration".to_string(),
            code: 2,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Invalid configuration".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "invalid_command".to_string(),
            code: 3,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Invalid command".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "invalid_format".to_string(),
            code: 4,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Invalid format".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "feature_unavailable".to_string(),
            code: 5,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Feature is unavailable".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_create_base_directory".to_string(),
            code: 10,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Cannot create base directory".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "resource_not_found".to_string(),
            code: 20,
            signature: "String".to_string(),
            converts_from: "".to_string(),
            template: "Resource with key: {0} was not found.".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_load_resource".to_string(),
            code: 21,
            signature: "String".to_string(),
            converts_from: "".to_string(),
            template: "Cannot load resource with key: {0}".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_save_resource".to_string(),
            code: 22,
            signature: "String".to_string(),
            converts_from: "".to_string(),
            template: "Cannot save resource with key: {0}".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_delete_resource".to_string(),
            code: 23,
            signature: "String".to_string(),
            converts_from: "".to_string(),
            template: "Cannot delete resource with key: {0}".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_serialize_resource".to_string(),
            code: 24,
            signature: "String".to_string(),
            converts_from: "".to_string(),
            template: "Cannot serialize resource with key: {0}".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_deserialize_resource".to_string(),
            code: 25,
            signature: "String".to_string(),
            converts_from: "".to_string(),
            template: "Cannot deserialize resource with key: {0}".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "unauthenticated".to_string(),
            code: 40,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Unauthenticated".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "unauthorized".to_string(),
            code: 41,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Unauthorized".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "invalid_credentials".to_string(),
            code: 42,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Invalid credentials".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "invalid_username".to_string(),
            code: 43,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Invalid username".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "invalid_password".to_string(),
            code: 44,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Invalid password".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "invalid_user_status".to_string(),
            code: 45,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Invalid user status".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "user_already_exists".to_string(),
            code: 46,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "User already exists".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "user_inactive".to_string(),
            code: 47,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "User inactive".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_delete_user".to_string(),
            code: 48,
            signature: "u32".to_string(),
            converts_from: "".to_string(),
            template: "Cannot delete user with ID: {0}".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_change_permissions".to_string(),
            code: 49,
            signature: "u32".to_string(),
            converts_from: "".to_string(),
            template: "Cannot change permissions for user with ID: {0}".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "not_connected".to_string(),
            code: 51,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Not connected".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "request_error".to_string(),
            code: 52,
            signature: "".to_string(),
            converts_from: "reqwest::Error".to_string(),
            template: "Request error".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "invalid_encryption_key".to_string(),
            code: 60,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Invalid encryption key".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_encrypt_data".to_string(),
            code: 61,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Cannot encrypt data".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_decrypt_data".to_string(),
            code: 62,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Cannot decrypt data".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "invalid_jwt_algorithm".to_string(),
            code: 63,
            signature: "String".to_string(),
            converts_from: "".to_string(),
            template: "Invalid JWT algorithm: {0}".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "invalid_jwt_secret".to_string(),
            code: 64,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Invalid JWT secret".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "client_not_found".to_string(),
            code: 100,
            signature: "u32".to_string(),
            converts_from: "".to_string(),
            template: "Client with ID: {0} was not found.".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "invalid_client_id".to_string(),
            code: 101,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Invalid client ID".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "io_error".to_string(),
            code: 200,
            signature: "".to_string(),
            converts_from: "io::Error".to_string(),
            template: "IO error".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "write_error".to_string(),
            code: 201,
            signature: "".to_string(),
            converts_from: "WriteError".to_string(),
            template: "Write error".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_parse_utf8".to_string(),
            code: 202,
            signature: "".to_string(),
            converts_from: "Utf8Error".to_string(),
            template: "Cannot parse UTF8".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_parse_int".to_string(),
            code: 203,
            signature: "".to_string(),
            converts_from: "ParseIntError".to_string(),
            template: "Cannot parse integer".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_parse_slice".to_string(),
            code: 204,
            signature: "".to_string(),
            converts_from: "TryFromSliceError".to_string(),
            template: "Cannot parse integer".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "http_response_error".to_string(),
            code: 300,
            signature: "u16, String".to_string(),
            converts_from: "".to_string(),
            template: "HTTP response error, status: {0}, body: {1}".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "request_middleware_error".to_string(),
            code: 301,
            signature: "".to_string(),
            converts_from: "reqwest_middleware::Error".to_string(),
            template: "Request middleware error".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_create_endpoint".to_string(),
            code: 302,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Cannot create endpoint".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_parse_url".to_string(),
            code: 303,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Cannot parse URL".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "invalid_response".to_string(),
            code: 304,
            signature: "u32".to_string(),
            converts_from: "".to_string(),
            template: "Invalid response: {0}".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "empty_response".to_string(),
            code: 305,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Empty response".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_parse_address".to_string(),
            code: 306,
            signature: "".to_string(),
            converts_from: "AddrParseError".to_string(),
            template: "Cannot parse address".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "read_error".to_string(),
            code: 307,
            signature: "".to_string(),
            converts_from: "ReadError".to_string(),
            template: "Read error".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "connection_error".to_string(),
            code: 308,
            signature: "".to_string(),
            converts_from: "ConnectionError".to_string(),
            template: "Connection error".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "read_to_end_error".to_string(),
            code: 309,
            signature: "".to_string(),
            converts_from: "ReadToEndError".to_string(),
            template: "Read to end error".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_create_streams_directory".to_string(),
            code: 1000,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Cannot create streams directory".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_create_stream_directory".to_string(),
            code: 1001,
            signature: "u32".to_string(),
            converts_from: "".to_string(),
            template: "Cannot create stream with ID: {0} directory".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_create_stream_info".to_string(),
            code: 1002,
            signature: "u32".to_string(),
            converts_from: "".to_string(),
            template: "Failed to create stream info file for stream with ID: {0}".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_update_stream_info".to_string(),
            code: 1003,
            signature: "u32".to_string(),
            converts_from: "".to_string(),
            template: "Failed to update stream info for stream with ID: {0}".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_open_stream_info".to_string(),
            code: 1004,
            signature: "u32".to_string(),
            converts_from: "".to_string(),
            template: "Failed to open stream info file for stream with ID: {0}".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_read_stream_info".to_string(),
            code: 1005,
            signature: "u32".to_string(),
            converts_from: "".to_string(),
            template: "Failed to read stream info file for stream with ID: {0}".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_create_stream".to_string(),
            code: 1006,
            signature: "u32".to_string(),
            converts_from: "".to_string(),
            template: "Failed to create stream with ID: {0}".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_delete_stream".to_string(),
            code: 1007,
            signature: "u32".to_string(),
            converts_from: "".to_string(),
            template: "Failed to delete stream with ID: {0}".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_delete_stream_directory".to_string(),
            code: 1008,
            signature: "u32".to_string(),
            converts_from: "".to_string(),
            template: "Failed to delete stream directory with ID: {0}".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "stream_id_not_found".to_string(),
            code: 1009,
            signature: "u32".to_string(),
            converts_from: "".to_string(),
            template: "Stream with ID: {0} was not found.".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "stream_name_not_found".to_string(),
            code: 1010,
            signature: "String".to_string(),
            converts_from: "".to_string(),
            template: "Stream with name: {0} was not found.".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "stream_id_already_exists".to_string(),
            code: 1011,
            signature: "u32".to_string(),
            converts_from: "".to_string(),
            template: "Stream with ID: {0} already exists.".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "stream_name_already_exists".to_string(),
            code: 1012,
            signature: "String".to_string(),
            converts_from: "".to_string(),
            template: "Stream with name: {0} already exists.".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "invalid_stream_name".to_string(),
            code: 1013,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Invalid stream name".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "invalid_stream_id".to_string(),
            code: 1014,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Invalid stream ID".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_read_streams".to_string(),
            code: 1015,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Cannot read streams".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_create_topics_directory".to_string(),
            code: 2000,
            signature: "u32".to_string(),
            converts_from: "".to_string(),
            template: "Cannot create topics directory for stream with ID: {0}".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_create_topic_directory".to_string(),
            code: 2001,
            signature: "u32, u32".to_string(),
            converts_from: "".to_string(),
            template: "Failed to create directory for topic with ID: {0} for stream with ID: {1}.".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_create_topic_info".to_string(),
            code: 2002,
            signature: "u32, u32".to_string(),
            converts_from: "".to_string(),
            template: "Failed to create topic info file for topic with ID: {0} for stream with ID: {1}.".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_update_topic_info".to_string(),
            code: 2003,
            signature: "u32, u32".to_string(),
            converts_from: "".to_string(),
            template: "Failed to update topic info for topic with ID: {0} for stream with ID: {1}.".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_open_topic_info".to_string(),
            code: 2004,
            signature: "u32, u32".to_string(),
            converts_from: "".to_string(),
            template: "Failed to open topic info file for topic with ID: {0} for stream with ID: {1}.".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_read_topic_info".to_string(),
            code: 2005,
            signature: "u32, u32".to_string(),
            converts_from: "".to_string(),
            template: "Failed to read topic info file for topic with ID: {0} for stream with ID: {1}.".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_create_topic".to_string(),
            code: 2006,
            signature: "u32, u32".to_string(),
            converts_from: "".to_string(),
            template: "Failed to create topic with ID: {0} for stream with ID: {1}.".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_delete_topic".to_string(),
            code: 2007,
            signature: "u32, u32".to_string(),
            converts_from: "".to_string(),
            template: "Failed to delete topic with ID: {0} for stream with ID: {1}.".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_delete_topic_directory".to_string(),
            code: 2008,
            signature: "u32, u32".to_string(),
            converts_from: "".to_string(),
            template: "Failed to delete topic directory with ID: {0} for stream with ID: {1}.".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_poll_topic".to_string(),
            code: 2009,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Cannot poll topic".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "topic_id_not_found".to_string(),
            code: 2010,
            signature: "u32, u32".to_string(),
            converts_from: "".to_string(),
            template: "Topic with ID: {0} for stream with ID: {1} was not found.".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "topic_name_not_found".to_string(),
            code: 2011,
            signature: "String, u32".to_string(),
            converts_from: "".to_string(),
            template: "Topic with name: {0} for stream with ID: {1} was not found.".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "topic_id_already_exists".to_string(),
            code: 2012,
            signature: "u32, u32".to_string(),
            converts_from: "".to_string(),
            template: "Topic with ID: {0} for stream with ID: {1} already exists.".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "topic_name_already_exists".to_string(),
            code: 2013,
            signature: "String, u32".to_string(),
            converts_from: "".to_string(),
            template: "Topic with name: {0} for stream with ID: {1} already exists.".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "invalid_topic_name".to_string(),
            code: 2014,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Invalid topic name".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "too_many_partitions".to_string(),
            code: 2015,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Too many partitions".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "invalid_topic_id".to_string(),
            code: 2016,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Invalid topic ID".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_read_topics".to_string(),
            code: 2017,
            signature: "u32".to_string(),
            converts_from: "".to_string(),
            template: "Cannot read topics for stream with ID: {0}".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_create_partition".to_string(),
            code: 3000,
            signature: "u32, u32, u32".to_string(),
            converts_from: "".to_string(),
            template: "Cannot create partition with ID: {0} for stream with ID: {1} and topic with ID: {2}".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_create_partitions_directory".to_string(),
            code: 3001,
            signature: "u32, u32".to_string(),
            converts_from: "".to_string(),
            template: "Failed to create directory for partitions for stream with ID: {0} and topic with ID: {1}".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_create_partition_directory".to_string(),
            code: 3002,
            signature: "u32, u32, u32".to_string(),
            converts_from: "".to_string(),
            template: "Failed to create directory for partition with ID: {0} for stream with ID: {1} and topic with ID: {2}".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_open_partition_log_file".to_string(),
            code: 3003,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Cannot open partition log file".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_read_partitions".to_string(),
            code: 3004,
            signature: "u32, u32".to_string(),
            converts_from: "".to_string(),
            template: "Failed to read partitions directories for topic with ID: {0} and stream with ID: {1}".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_delete_partition".to_string(),
            code: 3005,
            signature: "u32, u32, u32".to_string(),
            converts_from: "".to_string(),
            template: "Failed to delete partition with ID: {0} for stream with ID: {1} and topic with ID: {2}".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_delete_partition_directory".to_string(),
            code: 3006,
            signature: "u32, u32, u32".to_string(),
            converts_from: "".to_string(),
            template: "Failed to delete partition directory with ID: {0} for stream with ID: {1} and topic with ID: {2}".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "partition_not_found".to_string(),
            code: 3007,
            signature: "u32, u32, u32".to_string(),
            converts_from: "".to_string(),
            template: "Partition with ID: {0} for topic with ID: {1} for stream with ID: {2} was not found.".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "no_partitions".to_string(),
            code: 3008,
            signature: "u32, u32".to_string(),
            converts_from: "".to_string(),
            template: "Topic with ID: {0} for stream with ID: {1} has no partitions.".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "segment_not_found".to_string(),
            code: 4000,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Segment not found".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "segment_closed".to_string(),
            code: 4001,
            signature: "u64, u32".to_string(),
            converts_from: "".to_string(),
            template: "Segment with start offset: {0} and partition with ID: {1} is closed".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "invalid_segment_size".to_string(),
            code: 4002,
            signature: "u64".to_string(),
            converts_from: "".to_string(),
            template: "Segment size is invalid".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_create_segment_log_file".to_string(),
            code: 4003,
            signature: "String".to_string(),
            converts_from: "".to_string(),
            template: "Failed to create segment log file for path: {0}.".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_create_segment_index_file".to_string(),
            code: 4004,
            signature: "String".to_string(),
            converts_from: "".to_string(),
            template: "Failed to create segment index file for path: {0}.".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_create_segment_time_index_file".to_string(),
            code: 4005,
            signature: "String".to_string(),
            converts_from: "".to_string(),
            template: "Failed to create segment time index file for path: {0}.".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_save_messages_to_segment".to_string(),
            code: 4006,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Cannot save messages to segment".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_save_index_to_segment".to_string(),
            code: 4007,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Cannot save index to segment".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_save_time_index_to_segment".to_string(),
            code: 4008,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Cannot save time index to segment".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "invalid_messages_count".to_string(),
            code: 4009,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Invalid messages count".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_append_message".to_string(),
            code: 4010,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Cannot append message".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_read_message".to_string(),
            code: 4011,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Cannot read message".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_read_message_id".to_string(),
            code: 4012,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Cannot read message ID".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_read_message_state".to_string(),
            code: 4013,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Cannot read message state".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_read_message_timestamp".to_string(),
            code: 4014,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Cannot read message timestamp".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_read_headers_length".to_string(),
            code: 4015,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Cannot read headers length".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_read_headers_payload".to_string(),
            code: 4016,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Cannot read headers payload".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "too_big_headers_payload".to_string(),
            code: 4017,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Too big headers payload".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "invalid_header_key".to_string(),
            code: 4018,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Invalid header key".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "invalid_header_value".to_string(),
            code: 4019,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Invalid header value".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_read_message_length".to_string(),
            code: 4020,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Cannot read message length".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_read_message_payload".to_string(),
            code: 4021,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Cannot save messages to segment".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "too_big_message_payload".to_string(),
            code: 4022,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Too big message payload".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "too_many_messages".to_string(),
            code: 4023,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Too many messages".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "empty_message_payload".to_string(),
            code: 4024,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Empty message payload".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "invalid_message_payload_length".to_string(),
            code: 4025,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Invalid message payload length".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_read_message_checksum".to_string(),
            code: 4026,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Cannot read message checksum".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "invalid_message_checksum".to_string(),
            code: 4027,
            signature: "u32, u32, u64".to_string(),
            converts_from: "".to_string(),
            template: "Invalid message checksum: {0}, expected: {1}, for offset: {2}".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "invalid_key_value_length".to_string(),
            code: 4028,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Invalid key value length".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "invalid_offset".to_string(),
            code: 4100,
            signature: "u64".to_string(),
            converts_from: "".to_string(),
            template: "Invalid offset: {0}".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_read_consumer_offsets".to_string(),
            code: 4101,
            signature: "u32".to_string(),
            converts_from: "".to_string(),
            template: "Failed to read consumers offsets  for partition with ID: {0}".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "consumer_group_not_found".to_string(),
            code: 5000,
            signature: "u32, u32".to_string(),
            converts_from: "".to_string(),
            template: "Consumer group with ID: {0} for topic with ID: {1} was not found.".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "consumer_group_already_exists".to_string(),
            code: 5001,
            signature: "u32, u32".to_string(),
            converts_from: "".to_string(),
            template: "Consumer group with ID: {0} for topic with ID: {1} already exists.".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "consumer_group_member_not_found".to_string(),
            code: 5002,
            signature: "u32, u32, u32".to_string(),
            converts_from: "".to_string(),
            template: "Consumer group member with ID: {0} for group with ID: {1} for topic with ID: {2} was not found.".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "invalid_consumer_group_id".to_string(),
            code: 5003,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Invalid consumer group ID".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "invalid_consumer_group_name".to_string(),
            code: 5004,
            signature: "".to_string(),
            converts_from: "".to_string(),
            template: "Invalid consumer group name".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "consumer_group_name_already_exists".to_string(),
            code: 5005,
            signature: "String".to_string(),
            converts_from: "".to_string(),
            template: "Consumer group with name: {0} already exists.".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_create_consumer_group_info".to_string(),
            code: 5006,
            signature: "u32, u32, u32".to_string(),
            converts_from: "".to_string(),
            template: "Failed to create consumer group info file for ID: {0} for topic with ID: {1} for stream with ID: {2}.".to_string(),
        },
        ErrorRepositoryEntry {
            snake_case_name: "cannot_delete_consumer_group_info".to_string(),
            code: 5007,
            signature: "u32, u32, u32".to_string(),
            converts_from: "".to_string(),
            template: "Failed to delete consumer group info file for ID: {0} for topic with ID: {1} for stream with ID: {2}.".to_string(),
        },
    ];

    error_codes
}

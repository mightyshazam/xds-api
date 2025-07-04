impl serde::Serialize for AuthorizationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.allowed_headers.is_some() {
            len += 1;
        }
        if !self.headers_to_add.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.ext_authz.v3.AuthorizationRequest", len)?;
        if let Some(v) = self.allowed_headers.as_ref() {
            struct_ser.serialize_field("allowed_headers", v)?;
        }
        if !self.headers_to_add.is_empty() {
            struct_ser.serialize_field("headers_to_add", &self.headers_to_add)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AuthorizationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "allowed_headers",
            "allowedHeaders",
            "headers_to_add",
            "headersToAdd",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AllowedHeaders,
            HeadersToAdd,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "allowedHeaders" | "allowed_headers" => Ok(GeneratedField::AllowedHeaders),
                            "headersToAdd" | "headers_to_add" => Ok(GeneratedField::HeadersToAdd),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuthorizationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.ext_authz.v3.AuthorizationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AuthorizationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut allowed_headers__ = None;
                let mut headers_to_add__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AllowedHeaders => {
                            if allowed_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedHeaders"));
                            }
                            allowed_headers__ = map_.next_value()?;
                        }
                        GeneratedField::HeadersToAdd => {
                            if headers_to_add__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headersToAdd"));
                            }
                            headers_to_add__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AuthorizationRequest {
                    allowed_headers: allowed_headers__,
                    headers_to_add: headers_to_add__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.ext_authz.v3.AuthorizationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AuthorizationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.allowed_upstream_headers.is_some() {
            len += 1;
        }
        if self.allowed_upstream_headers_to_append.is_some() {
            len += 1;
        }
        if self.allowed_client_headers.is_some() {
            len += 1;
        }
        if self.allowed_client_headers_on_success.is_some() {
            len += 1;
        }
        if self.dynamic_metadata_from_headers.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.ext_authz.v3.AuthorizationResponse", len)?;
        if let Some(v) = self.allowed_upstream_headers.as_ref() {
            struct_ser.serialize_field("allowed_upstream_headers", v)?;
        }
        if let Some(v) = self.allowed_upstream_headers_to_append.as_ref() {
            struct_ser.serialize_field("allowed_upstream_headers_to_append", v)?;
        }
        if let Some(v) = self.allowed_client_headers.as_ref() {
            struct_ser.serialize_field("allowed_client_headers", v)?;
        }
        if let Some(v) = self.allowed_client_headers_on_success.as_ref() {
            struct_ser.serialize_field("allowed_client_headers_on_success", v)?;
        }
        if let Some(v) = self.dynamic_metadata_from_headers.as_ref() {
            struct_ser.serialize_field("dynamic_metadata_from_headers", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AuthorizationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "allowed_upstream_headers",
            "allowedUpstreamHeaders",
            "allowed_upstream_headers_to_append",
            "allowedUpstreamHeadersToAppend",
            "allowed_client_headers",
            "allowedClientHeaders",
            "allowed_client_headers_on_success",
            "allowedClientHeadersOnSuccess",
            "dynamic_metadata_from_headers",
            "dynamicMetadataFromHeaders",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AllowedUpstreamHeaders,
            AllowedUpstreamHeadersToAppend,
            AllowedClientHeaders,
            AllowedClientHeadersOnSuccess,
            DynamicMetadataFromHeaders,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "allowedUpstreamHeaders" | "allowed_upstream_headers" => Ok(GeneratedField::AllowedUpstreamHeaders),
                            "allowedUpstreamHeadersToAppend" | "allowed_upstream_headers_to_append" => Ok(GeneratedField::AllowedUpstreamHeadersToAppend),
                            "allowedClientHeaders" | "allowed_client_headers" => Ok(GeneratedField::AllowedClientHeaders),
                            "allowedClientHeadersOnSuccess" | "allowed_client_headers_on_success" => Ok(GeneratedField::AllowedClientHeadersOnSuccess),
                            "dynamicMetadataFromHeaders" | "dynamic_metadata_from_headers" => Ok(GeneratedField::DynamicMetadataFromHeaders),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuthorizationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.ext_authz.v3.AuthorizationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AuthorizationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut allowed_upstream_headers__ = None;
                let mut allowed_upstream_headers_to_append__ = None;
                let mut allowed_client_headers__ = None;
                let mut allowed_client_headers_on_success__ = None;
                let mut dynamic_metadata_from_headers__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AllowedUpstreamHeaders => {
                            if allowed_upstream_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedUpstreamHeaders"));
                            }
                            allowed_upstream_headers__ = map_.next_value()?;
                        }
                        GeneratedField::AllowedUpstreamHeadersToAppend => {
                            if allowed_upstream_headers_to_append__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedUpstreamHeadersToAppend"));
                            }
                            allowed_upstream_headers_to_append__ = map_.next_value()?;
                        }
                        GeneratedField::AllowedClientHeaders => {
                            if allowed_client_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedClientHeaders"));
                            }
                            allowed_client_headers__ = map_.next_value()?;
                        }
                        GeneratedField::AllowedClientHeadersOnSuccess => {
                            if allowed_client_headers_on_success__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedClientHeadersOnSuccess"));
                            }
                            allowed_client_headers_on_success__ = map_.next_value()?;
                        }
                        GeneratedField::DynamicMetadataFromHeaders => {
                            if dynamic_metadata_from_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicMetadataFromHeaders"));
                            }
                            dynamic_metadata_from_headers__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AuthorizationResponse {
                    allowed_upstream_headers: allowed_upstream_headers__,
                    allowed_upstream_headers_to_append: allowed_upstream_headers_to_append__,
                    allowed_client_headers: allowed_client_headers__,
                    allowed_client_headers_on_success: allowed_client_headers_on_success__,
                    dynamic_metadata_from_headers: dynamic_metadata_from_headers__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.ext_authz.v3.AuthorizationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BufferSettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_request_bytes != 0 {
            len += 1;
        }
        if self.allow_partial_message {
            len += 1;
        }
        if self.pack_as_bytes {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.ext_authz.v3.BufferSettings", len)?;
        if self.max_request_bytes != 0 {
            struct_ser.serialize_field("max_request_bytes", &self.max_request_bytes)?;
        }
        if self.allow_partial_message {
            struct_ser.serialize_field("allow_partial_message", &self.allow_partial_message)?;
        }
        if self.pack_as_bytes {
            struct_ser.serialize_field("pack_as_bytes", &self.pack_as_bytes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BufferSettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_request_bytes",
            "maxRequestBytes",
            "allow_partial_message",
            "allowPartialMessage",
            "pack_as_bytes",
            "packAsBytes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxRequestBytes,
            AllowPartialMessage,
            PackAsBytes,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "maxRequestBytes" | "max_request_bytes" => Ok(GeneratedField::MaxRequestBytes),
                            "allowPartialMessage" | "allow_partial_message" => Ok(GeneratedField::AllowPartialMessage),
                            "packAsBytes" | "pack_as_bytes" => Ok(GeneratedField::PackAsBytes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BufferSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.ext_authz.v3.BufferSettings")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BufferSettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_request_bytes__ = None;
                let mut allow_partial_message__ = None;
                let mut pack_as_bytes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MaxRequestBytes => {
                            if max_request_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxRequestBytes"));
                            }
                            max_request_bytes__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AllowPartialMessage => {
                            if allow_partial_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowPartialMessage"));
                            }
                            allow_partial_message__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PackAsBytes => {
                            if pack_as_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("packAsBytes"));
                            }
                            pack_as_bytes__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BufferSettings {
                    max_request_bytes: max_request_bytes__.unwrap_or_default(),
                    allow_partial_message: allow_partial_message__.unwrap_or_default(),
                    pack_as_bytes: pack_as_bytes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.ext_authz.v3.BufferSettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckSettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.context_extensions.is_empty() {
            len += 1;
        }
        if self.disable_request_body_buffering {
            len += 1;
        }
        if self.with_request_body.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.ext_authz.v3.CheckSettings", len)?;
        if !self.context_extensions.is_empty() {
            struct_ser.serialize_field("context_extensions", &self.context_extensions)?;
        }
        if self.disable_request_body_buffering {
            struct_ser.serialize_field("disable_request_body_buffering", &self.disable_request_body_buffering)?;
        }
        if let Some(v) = self.with_request_body.as_ref() {
            struct_ser.serialize_field("with_request_body", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckSettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "context_extensions",
            "contextExtensions",
            "disable_request_body_buffering",
            "disableRequestBodyBuffering",
            "with_request_body",
            "withRequestBody",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ContextExtensions,
            DisableRequestBodyBuffering,
            WithRequestBody,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "contextExtensions" | "context_extensions" => Ok(GeneratedField::ContextExtensions),
                            "disableRequestBodyBuffering" | "disable_request_body_buffering" => Ok(GeneratedField::DisableRequestBodyBuffering),
                            "withRequestBody" | "with_request_body" => Ok(GeneratedField::WithRequestBody),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.ext_authz.v3.CheckSettings")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CheckSettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut context_extensions__ = None;
                let mut disable_request_body_buffering__ = None;
                let mut with_request_body__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ContextExtensions => {
                            if context_extensions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contextExtensions"));
                            }
                            context_extensions__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::DisableRequestBodyBuffering => {
                            if disable_request_body_buffering__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableRequestBodyBuffering"));
                            }
                            disable_request_body_buffering__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WithRequestBody => {
                            if with_request_body__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withRequestBody"));
                            }
                            with_request_body__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CheckSettings {
                    context_extensions: context_extensions__.unwrap_or_default(),
                    disable_request_body_buffering: disable_request_body_buffering__.unwrap_or_default(),
                    with_request_body: with_request_body__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.ext_authz.v3.CheckSettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExtAuthz {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.transport_api_version != 0 {
            len += 1;
        }
        if self.failure_mode_allow {
            len += 1;
        }
        if self.failure_mode_allow_header_add {
            len += 1;
        }
        if self.with_request_body.is_some() {
            len += 1;
        }
        if self.clear_route_cache {
            len += 1;
        }
        if self.status_on_error.is_some() {
            len += 1;
        }
        if self.validate_mutations {
            len += 1;
        }
        if !self.metadata_context_namespaces.is_empty() {
            len += 1;
        }
        if !self.typed_metadata_context_namespaces.is_empty() {
            len += 1;
        }
        if !self.route_metadata_context_namespaces.is_empty() {
            len += 1;
        }
        if !self.route_typed_metadata_context_namespaces.is_empty() {
            len += 1;
        }
        if self.filter_enabled.is_some() {
            len += 1;
        }
        if self.filter_enabled_metadata.is_some() {
            len += 1;
        }
        if self.deny_at_disable.is_some() {
            len += 1;
        }
        if self.include_peer_certificate {
            len += 1;
        }
        if !self.stat_prefix.is_empty() {
            len += 1;
        }
        if !self.bootstrap_metadata_labels_key.is_empty() {
            len += 1;
        }
        if self.allowed_headers.is_some() {
            len += 1;
        }
        if self.disallowed_headers.is_some() {
            len += 1;
        }
        if self.include_tls_session {
            len += 1;
        }
        if self.charge_cluster_response_stats.is_some() {
            len += 1;
        }
        if self.encode_raw_headers {
            len += 1;
        }
        if self.decoder_header_mutation_rules.is_some() {
            len += 1;
        }
        if self.enable_dynamic_metadata_ingestion.is_some() {
            len += 1;
        }
        if self.filter_metadata.is_some() {
            len += 1;
        }
        if self.emit_filter_state_stats {
            len += 1;
        }
        if self.services.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.ext_authz.v3.ExtAuthz", len)?;
        if self.transport_api_version != 0 {
            let v = super::super::super::super::super::config::core::v3::ApiVersion::try_from(self.transport_api_version)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.transport_api_version)))?;
            struct_ser.serialize_field("transport_api_version", &v)?;
        }
        if self.failure_mode_allow {
            struct_ser.serialize_field("failure_mode_allow", &self.failure_mode_allow)?;
        }
        if self.failure_mode_allow_header_add {
            struct_ser.serialize_field("failure_mode_allow_header_add", &self.failure_mode_allow_header_add)?;
        }
        if let Some(v) = self.with_request_body.as_ref() {
            struct_ser.serialize_field("with_request_body", v)?;
        }
        if self.clear_route_cache {
            struct_ser.serialize_field("clear_route_cache", &self.clear_route_cache)?;
        }
        if let Some(v) = self.status_on_error.as_ref() {
            struct_ser.serialize_field("status_on_error", v)?;
        }
        if self.validate_mutations {
            struct_ser.serialize_field("validate_mutations", &self.validate_mutations)?;
        }
        if !self.metadata_context_namespaces.is_empty() {
            struct_ser.serialize_field("metadata_context_namespaces", &self.metadata_context_namespaces)?;
        }
        if !self.typed_metadata_context_namespaces.is_empty() {
            struct_ser.serialize_field("typed_metadata_context_namespaces", &self.typed_metadata_context_namespaces)?;
        }
        if !self.route_metadata_context_namespaces.is_empty() {
            struct_ser.serialize_field("route_metadata_context_namespaces", &self.route_metadata_context_namespaces)?;
        }
        if !self.route_typed_metadata_context_namespaces.is_empty() {
            struct_ser.serialize_field("route_typed_metadata_context_namespaces", &self.route_typed_metadata_context_namespaces)?;
        }
        if let Some(v) = self.filter_enabled.as_ref() {
            struct_ser.serialize_field("filter_enabled", v)?;
        }
        if let Some(v) = self.filter_enabled_metadata.as_ref() {
            struct_ser.serialize_field("filter_enabled_metadata", v)?;
        }
        if let Some(v) = self.deny_at_disable.as_ref() {
            struct_ser.serialize_field("deny_at_disable", v)?;
        }
        if self.include_peer_certificate {
            struct_ser.serialize_field("include_peer_certificate", &self.include_peer_certificate)?;
        }
        if !self.stat_prefix.is_empty() {
            struct_ser.serialize_field("stat_prefix", &self.stat_prefix)?;
        }
        if !self.bootstrap_metadata_labels_key.is_empty() {
            struct_ser.serialize_field("bootstrap_metadata_labels_key", &self.bootstrap_metadata_labels_key)?;
        }
        if let Some(v) = self.allowed_headers.as_ref() {
            struct_ser.serialize_field("allowed_headers", v)?;
        }
        if let Some(v) = self.disallowed_headers.as_ref() {
            struct_ser.serialize_field("disallowed_headers", v)?;
        }
        if self.include_tls_session {
            struct_ser.serialize_field("include_tls_session", &self.include_tls_session)?;
        }
        if let Some(v) = self.charge_cluster_response_stats.as_ref() {
            struct_ser.serialize_field("charge_cluster_response_stats", v)?;
        }
        if self.encode_raw_headers {
            struct_ser.serialize_field("encode_raw_headers", &self.encode_raw_headers)?;
        }
        if let Some(v) = self.decoder_header_mutation_rules.as_ref() {
            struct_ser.serialize_field("decoder_header_mutation_rules", v)?;
        }
        if let Some(v) = self.enable_dynamic_metadata_ingestion.as_ref() {
            struct_ser.serialize_field("enable_dynamic_metadata_ingestion", v)?;
        }
        if let Some(v) = self.filter_metadata.as_ref() {
            struct_ser.serialize_field("filter_metadata", v)?;
        }
        if self.emit_filter_state_stats {
            struct_ser.serialize_field("emit_filter_state_stats", &self.emit_filter_state_stats)?;
        }
        if let Some(v) = self.services.as_ref() {
            match v {
                ext_authz::Services::GrpcService(v) => {
                    struct_ser.serialize_field("grpc_service", v)?;
                }
                ext_authz::Services::HttpService(v) => {
                    struct_ser.serialize_field("http_service", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExtAuthz {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "transport_api_version",
            "transportApiVersion",
            "failure_mode_allow",
            "failureModeAllow",
            "failure_mode_allow_header_add",
            "failureModeAllowHeaderAdd",
            "with_request_body",
            "withRequestBody",
            "clear_route_cache",
            "clearRouteCache",
            "status_on_error",
            "statusOnError",
            "validate_mutations",
            "validateMutations",
            "metadata_context_namespaces",
            "metadataContextNamespaces",
            "typed_metadata_context_namespaces",
            "typedMetadataContextNamespaces",
            "route_metadata_context_namespaces",
            "routeMetadataContextNamespaces",
            "route_typed_metadata_context_namespaces",
            "routeTypedMetadataContextNamespaces",
            "filter_enabled",
            "filterEnabled",
            "filter_enabled_metadata",
            "filterEnabledMetadata",
            "deny_at_disable",
            "denyAtDisable",
            "include_peer_certificate",
            "includePeerCertificate",
            "stat_prefix",
            "statPrefix",
            "bootstrap_metadata_labels_key",
            "bootstrapMetadataLabelsKey",
            "allowed_headers",
            "allowedHeaders",
            "disallowed_headers",
            "disallowedHeaders",
            "include_tls_session",
            "includeTlsSession",
            "charge_cluster_response_stats",
            "chargeClusterResponseStats",
            "encode_raw_headers",
            "encodeRawHeaders",
            "decoder_header_mutation_rules",
            "decoderHeaderMutationRules",
            "enable_dynamic_metadata_ingestion",
            "enableDynamicMetadataIngestion",
            "filter_metadata",
            "filterMetadata",
            "emit_filter_state_stats",
            "emitFilterStateStats",
            "grpc_service",
            "grpcService",
            "http_service",
            "httpService",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TransportApiVersion,
            FailureModeAllow,
            FailureModeAllowHeaderAdd,
            WithRequestBody,
            ClearRouteCache,
            StatusOnError,
            ValidateMutations,
            MetadataContextNamespaces,
            TypedMetadataContextNamespaces,
            RouteMetadataContextNamespaces,
            RouteTypedMetadataContextNamespaces,
            FilterEnabled,
            FilterEnabledMetadata,
            DenyAtDisable,
            IncludePeerCertificate,
            StatPrefix,
            BootstrapMetadataLabelsKey,
            AllowedHeaders,
            DisallowedHeaders,
            IncludeTlsSession,
            ChargeClusterResponseStats,
            EncodeRawHeaders,
            DecoderHeaderMutationRules,
            EnableDynamicMetadataIngestion,
            FilterMetadata,
            EmitFilterStateStats,
            GrpcService,
            HttpService,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "transportApiVersion" | "transport_api_version" => Ok(GeneratedField::TransportApiVersion),
                            "failureModeAllow" | "failure_mode_allow" => Ok(GeneratedField::FailureModeAllow),
                            "failureModeAllowHeaderAdd" | "failure_mode_allow_header_add" => Ok(GeneratedField::FailureModeAllowHeaderAdd),
                            "withRequestBody" | "with_request_body" => Ok(GeneratedField::WithRequestBody),
                            "clearRouteCache" | "clear_route_cache" => Ok(GeneratedField::ClearRouteCache),
                            "statusOnError" | "status_on_error" => Ok(GeneratedField::StatusOnError),
                            "validateMutations" | "validate_mutations" => Ok(GeneratedField::ValidateMutations),
                            "metadataContextNamespaces" | "metadata_context_namespaces" => Ok(GeneratedField::MetadataContextNamespaces),
                            "typedMetadataContextNamespaces" | "typed_metadata_context_namespaces" => Ok(GeneratedField::TypedMetadataContextNamespaces),
                            "routeMetadataContextNamespaces" | "route_metadata_context_namespaces" => Ok(GeneratedField::RouteMetadataContextNamespaces),
                            "routeTypedMetadataContextNamespaces" | "route_typed_metadata_context_namespaces" => Ok(GeneratedField::RouteTypedMetadataContextNamespaces),
                            "filterEnabled" | "filter_enabled" => Ok(GeneratedField::FilterEnabled),
                            "filterEnabledMetadata" | "filter_enabled_metadata" => Ok(GeneratedField::FilterEnabledMetadata),
                            "denyAtDisable" | "deny_at_disable" => Ok(GeneratedField::DenyAtDisable),
                            "includePeerCertificate" | "include_peer_certificate" => Ok(GeneratedField::IncludePeerCertificate),
                            "statPrefix" | "stat_prefix" => Ok(GeneratedField::StatPrefix),
                            "bootstrapMetadataLabelsKey" | "bootstrap_metadata_labels_key" => Ok(GeneratedField::BootstrapMetadataLabelsKey),
                            "allowedHeaders" | "allowed_headers" => Ok(GeneratedField::AllowedHeaders),
                            "disallowedHeaders" | "disallowed_headers" => Ok(GeneratedField::DisallowedHeaders),
                            "includeTlsSession" | "include_tls_session" => Ok(GeneratedField::IncludeTlsSession),
                            "chargeClusterResponseStats" | "charge_cluster_response_stats" => Ok(GeneratedField::ChargeClusterResponseStats),
                            "encodeRawHeaders" | "encode_raw_headers" => Ok(GeneratedField::EncodeRawHeaders),
                            "decoderHeaderMutationRules" | "decoder_header_mutation_rules" => Ok(GeneratedField::DecoderHeaderMutationRules),
                            "enableDynamicMetadataIngestion" | "enable_dynamic_metadata_ingestion" => Ok(GeneratedField::EnableDynamicMetadataIngestion),
                            "filterMetadata" | "filter_metadata" => Ok(GeneratedField::FilterMetadata),
                            "emitFilterStateStats" | "emit_filter_state_stats" => Ok(GeneratedField::EmitFilterStateStats),
                            "grpcService" | "grpc_service" => Ok(GeneratedField::GrpcService),
                            "httpService" | "http_service" => Ok(GeneratedField::HttpService),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExtAuthz;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.ext_authz.v3.ExtAuthz")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExtAuthz, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut transport_api_version__ = None;
                let mut failure_mode_allow__ = None;
                let mut failure_mode_allow_header_add__ = None;
                let mut with_request_body__ = None;
                let mut clear_route_cache__ = None;
                let mut status_on_error__ = None;
                let mut validate_mutations__ = None;
                let mut metadata_context_namespaces__ = None;
                let mut typed_metadata_context_namespaces__ = None;
                let mut route_metadata_context_namespaces__ = None;
                let mut route_typed_metadata_context_namespaces__ = None;
                let mut filter_enabled__ = None;
                let mut filter_enabled_metadata__ = None;
                let mut deny_at_disable__ = None;
                let mut include_peer_certificate__ = None;
                let mut stat_prefix__ = None;
                let mut bootstrap_metadata_labels_key__ = None;
                let mut allowed_headers__ = None;
                let mut disallowed_headers__ = None;
                let mut include_tls_session__ = None;
                let mut charge_cluster_response_stats__ = None;
                let mut encode_raw_headers__ = None;
                let mut decoder_header_mutation_rules__ = None;
                let mut enable_dynamic_metadata_ingestion__ = None;
                let mut filter_metadata__ = None;
                let mut emit_filter_state_stats__ = None;
                let mut services__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TransportApiVersion => {
                            if transport_api_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transportApiVersion"));
                            }
                            transport_api_version__ = Some(map_.next_value::<super::super::super::super::super::config::core::v3::ApiVersion>()? as i32);
                        }
                        GeneratedField::FailureModeAllow => {
                            if failure_mode_allow__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failureModeAllow"));
                            }
                            failure_mode_allow__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FailureModeAllowHeaderAdd => {
                            if failure_mode_allow_header_add__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failureModeAllowHeaderAdd"));
                            }
                            failure_mode_allow_header_add__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WithRequestBody => {
                            if with_request_body__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withRequestBody"));
                            }
                            with_request_body__ = map_.next_value()?;
                        }
                        GeneratedField::ClearRouteCache => {
                            if clear_route_cache__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clearRouteCache"));
                            }
                            clear_route_cache__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StatusOnError => {
                            if status_on_error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statusOnError"));
                            }
                            status_on_error__ = map_.next_value()?;
                        }
                        GeneratedField::ValidateMutations => {
                            if validate_mutations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validateMutations"));
                            }
                            validate_mutations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MetadataContextNamespaces => {
                            if metadata_context_namespaces__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataContextNamespaces"));
                            }
                            metadata_context_namespaces__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TypedMetadataContextNamespaces => {
                            if typed_metadata_context_namespaces__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedMetadataContextNamespaces"));
                            }
                            typed_metadata_context_namespaces__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RouteMetadataContextNamespaces => {
                            if route_metadata_context_namespaces__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routeMetadataContextNamespaces"));
                            }
                            route_metadata_context_namespaces__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RouteTypedMetadataContextNamespaces => {
                            if route_typed_metadata_context_namespaces__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routeTypedMetadataContextNamespaces"));
                            }
                            route_typed_metadata_context_namespaces__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FilterEnabled => {
                            if filter_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterEnabled"));
                            }
                            filter_enabled__ = map_.next_value()?;
                        }
                        GeneratedField::FilterEnabledMetadata => {
                            if filter_enabled_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterEnabledMetadata"));
                            }
                            filter_enabled_metadata__ = map_.next_value()?;
                        }
                        GeneratedField::DenyAtDisable => {
                            if deny_at_disable__.is_some() {
                                return Err(serde::de::Error::duplicate_field("denyAtDisable"));
                            }
                            deny_at_disable__ = map_.next_value()?;
                        }
                        GeneratedField::IncludePeerCertificate => {
                            if include_peer_certificate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includePeerCertificate"));
                            }
                            include_peer_certificate__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StatPrefix => {
                            if stat_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statPrefix"));
                            }
                            stat_prefix__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BootstrapMetadataLabelsKey => {
                            if bootstrap_metadata_labels_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bootstrapMetadataLabelsKey"));
                            }
                            bootstrap_metadata_labels_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AllowedHeaders => {
                            if allowed_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedHeaders"));
                            }
                            allowed_headers__ = map_.next_value()?;
                        }
                        GeneratedField::DisallowedHeaders => {
                            if disallowed_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disallowedHeaders"));
                            }
                            disallowed_headers__ = map_.next_value()?;
                        }
                        GeneratedField::IncludeTlsSession => {
                            if include_tls_session__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeTlsSession"));
                            }
                            include_tls_session__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ChargeClusterResponseStats => {
                            if charge_cluster_response_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chargeClusterResponseStats"));
                            }
                            charge_cluster_response_stats__ = map_.next_value()?;
                        }
                        GeneratedField::EncodeRawHeaders => {
                            if encode_raw_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("encodeRawHeaders"));
                            }
                            encode_raw_headers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DecoderHeaderMutationRules => {
                            if decoder_header_mutation_rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("decoderHeaderMutationRules"));
                            }
                            decoder_header_mutation_rules__ = map_.next_value()?;
                        }
                        GeneratedField::EnableDynamicMetadataIngestion => {
                            if enable_dynamic_metadata_ingestion__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableDynamicMetadataIngestion"));
                            }
                            enable_dynamic_metadata_ingestion__ = map_.next_value()?;
                        }
                        GeneratedField::FilterMetadata => {
                            if filter_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterMetadata"));
                            }
                            filter_metadata__ = map_.next_value()?;
                        }
                        GeneratedField::EmitFilterStateStats => {
                            if emit_filter_state_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emitFilterStateStats"));
                            }
                            emit_filter_state_stats__ = Some(map_.next_value()?);
                        }
                        GeneratedField::GrpcService => {
                            if services__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grpcService"));
                            }
                            services__ = map_.next_value::<::std::option::Option<_>>()?.map(ext_authz::Services::GrpcService)
;
                        }
                        GeneratedField::HttpService => {
                            if services__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpService"));
                            }
                            services__ = map_.next_value::<::std::option::Option<_>>()?.map(ext_authz::Services::HttpService)
;
                        }
                    }
                }
                Ok(ExtAuthz {
                    transport_api_version: transport_api_version__.unwrap_or_default(),
                    failure_mode_allow: failure_mode_allow__.unwrap_or_default(),
                    failure_mode_allow_header_add: failure_mode_allow_header_add__.unwrap_or_default(),
                    with_request_body: with_request_body__,
                    clear_route_cache: clear_route_cache__.unwrap_or_default(),
                    status_on_error: status_on_error__,
                    validate_mutations: validate_mutations__.unwrap_or_default(),
                    metadata_context_namespaces: metadata_context_namespaces__.unwrap_or_default(),
                    typed_metadata_context_namespaces: typed_metadata_context_namespaces__.unwrap_or_default(),
                    route_metadata_context_namespaces: route_metadata_context_namespaces__.unwrap_or_default(),
                    route_typed_metadata_context_namespaces: route_typed_metadata_context_namespaces__.unwrap_or_default(),
                    filter_enabled: filter_enabled__,
                    filter_enabled_metadata: filter_enabled_metadata__,
                    deny_at_disable: deny_at_disable__,
                    include_peer_certificate: include_peer_certificate__.unwrap_or_default(),
                    stat_prefix: stat_prefix__.unwrap_or_default(),
                    bootstrap_metadata_labels_key: bootstrap_metadata_labels_key__.unwrap_or_default(),
                    allowed_headers: allowed_headers__,
                    disallowed_headers: disallowed_headers__,
                    include_tls_session: include_tls_session__.unwrap_or_default(),
                    charge_cluster_response_stats: charge_cluster_response_stats__,
                    encode_raw_headers: encode_raw_headers__.unwrap_or_default(),
                    decoder_header_mutation_rules: decoder_header_mutation_rules__,
                    enable_dynamic_metadata_ingestion: enable_dynamic_metadata_ingestion__,
                    filter_metadata: filter_metadata__,
                    emit_filter_state_stats: emit_filter_state_stats__.unwrap_or_default(),
                    services: services__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.ext_authz.v3.ExtAuthz", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExtAuthzPerRoute {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#override.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.ext_authz.v3.ExtAuthzPerRoute", len)?;
        if let Some(v) = self.r#override.as_ref() {
            match v {
                ext_authz_per_route::Override::Disabled(v) => {
                    struct_ser.serialize_field("disabled", v)?;
                }
                ext_authz_per_route::Override::CheckSettings(v) => {
                    struct_ser.serialize_field("check_settings", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExtAuthzPerRoute {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "disabled",
            "check_settings",
            "checkSettings",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Disabled,
            CheckSettings,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "disabled" => Ok(GeneratedField::Disabled),
                            "checkSettings" | "check_settings" => Ok(GeneratedField::CheckSettings),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExtAuthzPerRoute;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.ext_authz.v3.ExtAuthzPerRoute")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExtAuthzPerRoute, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#override__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Disabled => {
                            if r#override__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disabled"));
                            }
                            r#override__ = map_.next_value::<::std::option::Option<_>>()?.map(ext_authz_per_route::Override::Disabled);
                        }
                        GeneratedField::CheckSettings => {
                            if r#override__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checkSettings"));
                            }
                            r#override__ = map_.next_value::<::std::option::Option<_>>()?.map(ext_authz_per_route::Override::CheckSettings)
;
                        }
                    }
                }
                Ok(ExtAuthzPerRoute {
                    r#override: r#override__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.ext_authz.v3.ExtAuthzPerRoute", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HttpService {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.server_uri.is_some() {
            len += 1;
        }
        if !self.path_prefix.is_empty() {
            len += 1;
        }
        if self.authorization_request.is_some() {
            len += 1;
        }
        if self.authorization_response.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.ext_authz.v3.HttpService", len)?;
        if let Some(v) = self.server_uri.as_ref() {
            struct_ser.serialize_field("server_uri", v)?;
        }
        if !self.path_prefix.is_empty() {
            struct_ser.serialize_field("path_prefix", &self.path_prefix)?;
        }
        if let Some(v) = self.authorization_request.as_ref() {
            struct_ser.serialize_field("authorization_request", v)?;
        }
        if let Some(v) = self.authorization_response.as_ref() {
            struct_ser.serialize_field("authorization_response", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpService {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "server_uri",
            "serverUri",
            "path_prefix",
            "pathPrefix",
            "authorization_request",
            "authorizationRequest",
            "authorization_response",
            "authorizationResponse",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ServerUri,
            PathPrefix,
            AuthorizationRequest,
            AuthorizationResponse,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "serverUri" | "server_uri" => Ok(GeneratedField::ServerUri),
                            "pathPrefix" | "path_prefix" => Ok(GeneratedField::PathPrefix),
                            "authorizationRequest" | "authorization_request" => Ok(GeneratedField::AuthorizationRequest),
                            "authorizationResponse" | "authorization_response" => Ok(GeneratedField::AuthorizationResponse),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HttpService;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.ext_authz.v3.HttpService")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HttpService, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut server_uri__ = None;
                let mut path_prefix__ = None;
                let mut authorization_request__ = None;
                let mut authorization_response__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ServerUri => {
                            if server_uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serverUri"));
                            }
                            server_uri__ = map_.next_value()?;
                        }
                        GeneratedField::PathPrefix => {
                            if path_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pathPrefix"));
                            }
                            path_prefix__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AuthorizationRequest => {
                            if authorization_request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorizationRequest"));
                            }
                            authorization_request__ = map_.next_value()?;
                        }
                        GeneratedField::AuthorizationResponse => {
                            if authorization_response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorizationResponse"));
                            }
                            authorization_response__ = map_.next_value()?;
                        }
                    }
                }
                Ok(HttpService {
                    server_uri: server_uri__,
                    path_prefix: path_prefix__.unwrap_or_default(),
                    authorization_request: authorization_request__,
                    authorization_response: authorization_response__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.ext_authz.v3.HttpService", FIELDS, GeneratedVisitor)
    }
}

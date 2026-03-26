use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct IndicatorSpecRepository {
    pub indicator_configs: HashMap<String, IndicatorConfig>,
    pub parameter_specs: HashMap<String, ParameterSpec>,
}

impl IndicatorSpecRepository {
    pub fn load_from_dir(path: &Path) -> Result<Self, SpecLoadError> {
        let indicator_dir = path.join("model").join("indicators");
        let mut indicator_configs = HashMap::new();
        let mut parameter_specs = HashMap::new();

        for entry in fs::read_dir(&indicator_dir).map_err(|source| SpecLoadError::Io {
            path: indicator_dir.clone(),
            source,
        })? {
            let entry = entry.map_err(|source| SpecLoadError::Io {
                path: indicator_dir.clone(),
                source,
            })?;
            let file_path = entry.path();
            if file_path.extension().and_then(|ext| ext.to_str()) != Some("yaml") {
                continue;
            }

            let yaml = fs::read_to_string(&file_path).map_err(|source| SpecLoadError::Io {
                path: file_path.clone(),
                source,
            })?;
            let document: SchemaDocument =
                serde_yaml::from_str(&yaml).map_err(|source| SpecLoadError::Yaml {
                    path: file_path.clone(),
                    source,
                })?;

            for (class_name, class_def) in &document.classes {
                if class_def.is_a.as_deref() == Some("IndicatorProductConfiguration") {
                    if let Some(config) =
                        build_indicator_config(class_name, class_def, &document.classes)
                    {
                        indicator_configs.insert(config.id.clone(), config);
                    }
                }
                if class_def.is_a.as_deref() == Some("ParameterSpecification") {
                    if let Some(parameter) =
                        build_parameter_spec(class_name, class_def, &document.classes)
                    {
                        parameter_specs.insert(parameter.id.clone(), parameter);
                    }
                }
            }
        }

        Ok(Self {
            indicator_configs,
            parameter_specs,
        })
    }
}

#[derive(Debug, Clone)]
pub struct IndicatorConfig {
    pub id: String,
    pub parameter_applications: Vec<ParameterApplication>,
}

#[derive(Debug, Clone)]
pub struct ParameterApplication {
    pub parameter_ref: String,
    pub weight: f64,
}

#[derive(Debug, Clone)]
pub struct ParameterSpec {
    pub id: String,
    pub questions: Vec<QuestionSpec>,
}

#[derive(Debug, Clone)]
pub struct QuestionSpec {
    pub id: String,
    pub weight: f64,
    pub answer_options: HashMap<String, f64>,
}

#[derive(Debug, Deserialize)]
struct SchemaDocument {
    classes: HashMap<String, ClassDefinition>,
}

#[derive(Debug, Deserialize)]
struct ClassDefinition {
    #[serde(default)]
    is_a: Option<String>,
    #[serde(default)]
    union_of: Vec<String>,
    #[serde(default)]
    attributes: HashMap<String, AttributeDefinition>,
}

#[derive(Debug, Deserialize)]
struct AttributeDefinition {
    #[serde(default)]
    range: Option<String>,
    #[serde(default)]
    ifabsent: Option<String>,
}

fn build_indicator_config(
    _class_name: &str,
    class_def: &ClassDefinition,
    classes: &HashMap<String, ClassDefinition>,
) -> Option<IndicatorConfig> {
    let id = parse_ifabsent_string(class_def.attributes.get("id")?.ifabsent.as_deref()?)?;
    let parameter_range = class_def
        .attributes
        .get("parameter_applications")?
        .range
        .as_deref()?;
    let parameter_applications = expand_union_members(parameter_range, classes)
        .into_iter()
        .filter_map(|member| {
            let class = classes.get(member)?;
            Some(ParameterApplication {
                parameter_ref: parse_ifabsent_string(
                    class.attributes.get("parameter_ref")?.ifabsent.as_deref()?,
                )?,
                weight: parse_ifabsent_float(class.attributes.get("weight")?.ifabsent.as_deref()?)?,
            })
        })
        .collect();

    Some(IndicatorConfig {
        id,
        parameter_applications,
    })
}

fn build_parameter_spec(
    _class_name: &str,
    class_def: &ClassDefinition,
    classes: &HashMap<String, ClassDefinition>,
) -> Option<ParameterSpec> {
    let id = parse_ifabsent_string(class_def.attributes.get("id")?.ifabsent.as_deref()?)?;
    let question_range = class_def.attributes.get("questions")?.range.as_deref()?;
    let questions = expand_union_members(question_range, classes)
        .into_iter()
        .filter_map(|member| build_question_spec(member, classes))
        .collect();
    Some(ParameterSpec { id, questions })
}

fn build_question_spec(
    class_name: &str,
    classes: &HashMap<String, ClassDefinition>,
) -> Option<QuestionSpec> {
    let class_def = classes.get(class_name)?;
    let id = parse_ifabsent_string(class_def.attributes.get("id")?.ifabsent.as_deref()?)?;
    let weight = parse_ifabsent_float(class_def.attributes.get("weight")?.ifabsent.as_deref()?)?;
    let answer_range = class_def
        .attributes
        .get("answer_options")?
        .range
        .as_deref()?;
    let answer_options = expand_union_members(answer_range, classes)
        .into_iter()
        .filter_map(|member| {
            let answer_class = classes.get(member)?;
            Some((
                parse_ifabsent_string(answer_class.attributes.get("id")?.ifabsent.as_deref()?)?,
                parse_ifabsent_float(answer_class.attributes.get("score")?.ifabsent.as_deref()?)?,
            ))
        })
        .collect();

    Some(QuestionSpec {
        id,
        weight,
        answer_options,
    })
}

fn expand_union_members<'a>(
    class_name: &'a str,
    classes: &'a HashMap<String, ClassDefinition>,
) -> Vec<&'a str> {
    let Some(class_def) = classes.get(class_name) else {
        return vec![class_name];
    };
    if class_def.union_of.is_empty() {
        vec![class_name]
    } else {
        class_def
            .union_of
            .iter()
            .flat_map(|member| expand_union_members(member, classes))
            .collect()
    }
}

fn parse_ifabsent_string(value: &str) -> Option<String> {
    value
        .strip_prefix("string(")
        .and_then(|v| v.strip_suffix(')'))
        .map(ToString::to_string)
}

fn parse_ifabsent_float(value: &str) -> Option<f64> {
    value
        .strip_prefix("float(")
        .and_then(|v| v.strip_suffix(')'))
        .and_then(|v| v.parse::<f64>().ok())
}

#[derive(Debug, thiserror::Error)]
pub enum SpecLoadError {
    #[error("I/O error reading {path}: {source}")]
    Io {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("YAML parse error in {path}: {source}")]
    Yaml {
        path: PathBuf,
        source: serde_yaml::Error,
    },
}

use std::collections::HashMap;

use handlebars::{
    Context,
    Handlebars,
    Helper,
    HelperResult,
    Output,
    PathAndJson,
    Renderable,
    RenderContext,
    RenderError,
    Template,
};

use serde::Serialize;

use serde_json::value::{
    Value as Json,
    to_value,
};

pub fn sorted(h: &Helper, r: &Handlebars, ctx: &Context, rc: &mut RenderContext, out: &mut Output) -> HelperResult {
    let container = h.param(0).ok_or_else(|| RenderError::new("expected container param"))?;
    let sort_key_param = h.param(1).ok_or_else(|| RenderError::new("expected sort key param"))?;

    let sort_key = match sort_key_param.value().as_str() {
        Some(s) => s,
        None => return Err(RenderError::new("sort key must be string")),
    };

    let template = match h.template() {
        Some(t) => t,
        None => return Ok(()),
    };

    match container.value() {
        Json::Array(ref list) => {
            let sorted = sort_by_key(list.iter().enumerate(), sort_key);
            sorted_content(sorted, container, h, r, ctx, rc, out, template)
        },
        Json::Object(ref obj) => {
            let sorted = sort_by_key(obj.iter(), sort_key);
            sorted_content(sorted, container, h, r, ctx, rc, out, template)
        },
        v => Err(RenderError::new(format!("Param type is not iterable: {:?}", v))),
    }

}

fn to_json<S>(value: S) -> Json
    where S: Serialize
{
    to_value(value).unwrap_or_default()
}

fn sort_by_key<'a, I, T>(iter: I, sort_key: &str) -> Vec<(T, &'a Json)>
    where
        I: Iterator<Item=(T, &'a Json)>
{
    let mut sorted = iter.collect::<Vec<_>>();
    sorted.sort_by(|(_, a), (_, b)| {
        let av = a.as_object().as_ref().and_then(|m| m.get(sort_key));
        let bv = b.as_object().as_ref().and_then(|m| m.get(sort_key));
        match (av, bv) {
            (Some(Json::Bool(u)), Some(Json::Bool(v))) => u.cmp(v),
            (Some(Json::String(u)), Some(Json::String(v))) => u.cmp(v),
            (Some(Json::Number(u)), Some(Json::Number(v))) => u.as_i64().cmp(&v.as_i64()),
            _ => std::cmp::Ordering::Less,
        }
    });
    sorted
}

fn sorted_content<'a, K>(
        sorted: Vec<(K, &'a Json)>,
        container: &PathAndJson,
        h: &Helper,
        r: &Handlebars,
        ctx: &Context,
        rc: &mut RenderContext,
        out: &mut Output,
        template: &Template,
    )
    -> HelperResult
    where
        K: Serialize + std::fmt::Display + Copy
{
    rc.promote_local_vars();
    let local_path_root =
        container
            .path_root()
            .map(|p| format!("{}/{}", rc.get_path(), p));

    let len = sorted.len();
    for (i, (k, v)) in sorted.into_iter().enumerate() {
        let mut local_rc = rc.derive();

        if let Some(ref p) = local_path_root {
            local_rc.push_local_path_root(p.clone());
        }

        local_rc.set_local_var("@key".to_string(), to_json(k));
        local_rc.set_local_var("@first".to_string(), to_json(i == 0));
        local_rc.set_local_var("@last".to_string(), to_json(i == len - 1));
        local_rc.set_local_var("@index".to_string(), to_json(i));

        if let Some(inner_path) = container.path() {
            let new_path = format!("{}/{}/[{}]", local_rc.get_path(), inner_path, k);
            local_rc.set_path(new_path);
        }

        if let Some((bp_key, bp_val)) = h.block_param_pair() {
            let mut map = HashMap::new();
            map.insert(bp_key.to_string(), to_json(k));
            map.insert(bp_val.to_string(), to_json(v));
            local_rc.push_block_context(&map)?;
        }

        template.render(r, ctx, &mut local_rc, out)?;

        if h.block_param().is_some() {
            local_rc.pop_block_context();
        }

        if local_path_root.is_some() {
            local_rc.pop_local_path_root();
        }
    }

    Ok(())
}

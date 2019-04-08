use std::collections::HashMap;

use handlebars::{
    Context,
    Handlebars,
    Helper,
    HelperResult,
    Output,
    Renderable,
    RenderContext,
    RenderError,
};

use serde_json::value::{
    Value as Json,
};

pub fn with_lookup(h: &Helper, r: &Handlebars, ctx: &Context, rc: &mut RenderContext, out: &mut Output) -> HelperResult {
    let container = h.param(0).ok_or_else(|| RenderError::new("expected container param"))?;
    let lookup_param = h.param(1).ok_or_else(|| RenderError::new("expected lookup param"))?;

    let template = match h.template() {
        Some(t) => t,
        None => return Ok(()),
    };

    match container.value() {
        Json::Array(ref list) => {
            lookup_param
                .value()
                .as_u64()
                .ok_or_else(|| RenderError::new(format!("Expected integer lookup param: {:?}", lookup_param)))
                .and_then(|v| {
                    list
                        .get(v as usize)
                        .or(Some(&Json::Null))
                        .ok_or_else(|| RenderError::new("unreachable".to_string()))
                })
        },
        Json::Object(ref obj) => {
            lookup_param
                .value()
                .as_str()
                .ok_or_else(|| RenderError::new(format!("Expected string lookup param: {:?}", lookup_param)))
                .and_then(|v| {
                    obj
                        .get(v)
                        .or(Some(&Json::Null))
                        .ok_or_else(|| RenderError::new("unreachable".to_string()))
                })
        },
        v => Err(RenderError::new(format!("Param type is not iterable: {:?}", v))),
    }.and_then(|lookup_value| {
        if lookup_value.is_null() {
            return Ok(());
        }

        let mut local_rc = rc.derive();
        rc.promote_local_vars();

        let local_path_root =
            container
            .path_root()
            .map(|p| format!("{}/{}", rc.get_path(), p));

        if let Some(ref p) = local_path_root {
            local_rc.push_local_path_root(p.clone());
        }

        /*
        if let Some(inner_path) = container.path() {
            let new_path = match lookup_param.value() {
                Json::String(s) => format!("{}/{}/[{}]", local_rc.get_path(), inner_path, s),
                Json::Number(n) => format!("{}/{}/[{}]", local_rc.get_path(), inner_path, n),
                _ => unreachable!(),
            };
            local_rc.set_path(new_path);
        }
        */

        if let Some((bp_key, bp_val)) = h.block_param_pair() {
            let mut map = HashMap::new();
            map.insert(bp_key.to_string(), lookup_param.value());
            map.insert(bp_val.to_string(), lookup_value);
            local_rc.push_block_context(&map)?;
        }

        template.render(r, ctx, &mut local_rc, out)?;

        if h.block_param().is_some() {
            local_rc.pop_block_context();
        }

        if local_path_root.is_some() {
            local_rc.pop_local_path_root();
        }

        Ok(())
    })
}

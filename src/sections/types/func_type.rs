use super::*;

/// https://webassembly.github.io/spec/core/syntax/types.html#function-types
#[derive(Debug, Clone)]
pub struct FunctionType {
    pub param_types: ParamsType,
    pub return_types: ResultType,
}

impl FunctionType {
    pub const ID: u32 = 0x60;

    /// Function id
    pub fn id() -> u32 {
        Self::ID
    }

    // [0x60, cnt, .., cnt, .., 0x60, cnt, .., cnt, ..]
    pub(crate) fn from_bytes(bytes: &[u8]) -> Self {
        // the most simple function like this => func() -> (), no param, no return
        // [0x60, 0, 0]
        assert!(bytes.len() >= 3);
        assert_eq!(bytes[0] as u32, Self::ID);

        // params
        let params_count = bytes[1] as usize;
        let param_types = {
            if params_count == 0 {
                vec![]
            } else {
                let mut param_types = Vec::with_capacity(params_count);
                let params_bytes = &bytes[2..2 + params_count];
                for ty in params_bytes {
                    let param = ValueType::NumType(NumberType::from(*ty));
                    param_types.push(param);
                }
                param_types
            }
        };

        // returns
        let return_bytes = &bytes[1 + 1 + params_count..];
        let return_count = return_bytes[0] as usize;
        let return_types = {
            if return_count == 0 {
                vec![]
            } else {
                let mut return_types = Vec::with_capacity(return_count);
                for ty in &return_bytes[1..] {
                    let _return = ValueType::NumType(NumberType::from(*ty));
                    return_types.push(_return);
                }
                return_types
            }
        };

        Self {
            param_types,
            return_types,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn function_no_param_no_return_should_work() {
        let func_bytes = [0x60, 0, 0u8];
        let func = FunctionType::from_bytes(&func_bytes);

        assert_eq!(func.param_types.len(), 0);
        assert_eq!(func.return_types.len(), 0);
    }

    #[test]
    fn function_no_param_one_return_should_work() {
        let func_bytes = [0x60, 0, 1, 0x7c];
        let func = FunctionType::from_bytes(&func_bytes);

        assert_eq!(func.param_types.len(), 0);
        assert_eq!(func.return_types.len(), 1);
    }

    #[test]
    fn function_one_param_no_return_should_work() {
        let func_bytes = [0x60, 1, 0x7c, 0];
        let func = FunctionType::from_bytes(&func_bytes);

        assert_eq!(func.param_types.len(), 1);
        assert_eq!(func.return_types.len(), 0);
    }

    #[test]
    fn function_four_params_four_return_should_work() {
        // In latest webassembly spec, the feature multi values is stablized.
        let func_bytes = [0x60, 4, 0x7c, 0x7d, 0x7e, 0x7f, 4, 0x7c, 0x7d, 0x7e, 0x7f];
        let func = FunctionType::from_bytes(&func_bytes);

        assert_eq!(func.param_types.len(), 4);
        assert_eq!(func.return_types.len(), 4);
    }
}

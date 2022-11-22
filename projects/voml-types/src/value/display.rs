use super::*;

impl Debug for Von {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Von::Integer(v) => Debug::fmt(v, f),
            Von::Decimal(v) => Debug::fmt(v, f),
            Von::List(v) => Debug::fmt(v, f),
            Von::Dict(v) => Debug::fmt(v, f),
        }
    }
}

impl Von {
    /// Get mutable reference if the value is dict
    ///
    ///
    /// # Arguments
    ///
    /// * `a`:
    ///
    /// returns: Option<&mut Dict<Von>>
    ///
    /// # Examples
    ///
    /// ```
    /// use voml_types::Von;
    /// ```
    #[inline]
    pub fn is_integer(&self) -> bool {
        match self {
            Von::Integer(_) => true,
            _ => false,
        }
    }
}

impl Von {
    /// Get mutable reference if the value is dict
    ///
    ///
    /// # Arguments
    ///
    /// * `a`:
    ///
    /// returns: Option<&mut Dict<Von>>
    ///
    /// # Examples
    ///
    /// ```
    /// use voml_types::Von;
    /// ```
    #[inline]
    pub fn get_integer(&self) -> Option<&Integer> {
        match self {
            Von::Integer(v) => Some(&**v),
            _ => None,
        }
    }
    /// Get mutable reference if the value is dict
    ///
    ///
    /// # Arguments
    ///
    /// * `a`:
    ///
    /// returns: Option<&mut Dict<Von>>
    ///
    /// # Examples
    ///
    /// ```
    /// use voml_types::Von;
    /// ```
    #[inline]
    pub fn mut_integer(&mut self) -> Option<&mut Integer> {
        match self {
            Von::Integer(v) => Some(&mut **v),
            _ => None,
        }
    }
    /// Get mutable reference if the value is dict
    ///
    ///
    /// # Arguments
    ///
    /// * `a`:
    ///
    /// returns: Option<&mut Dict<Von>>
    ///
    /// # Examples
    ///
    /// ```
    /// use voml_types::Von;
    /// ```
    #[inline]
    pub fn get_decimal(&self) -> Option<&Decimal> {
        match self {
            Von::Decimal(v) => Some(&**v),
            _ => None,
        }
    }
    /// Get mutable reference if the value is dict
    ///
    ///
    /// # Arguments
    ///
    /// * `a`:
    ///
    /// returns: Option<&mut Dict<Von>>
    ///
    /// # Examples
    ///
    /// ```
    /// use voml_types::Von;
    /// ```
    #[inline]
    pub fn mut_decimal(&mut self) -> Option<&mut Decimal> {
        match self {
            Von::Decimal(v) => Some(&mut **v),
            _ => None,
        }
    }
    /// Get mutable reference if the value is dict
    ///
    ///
    /// # Arguments
    ///
    /// * `a`:
    ///
    /// returns: Option<&mut Dict<Von>>
    ///
    /// # Examples
    ///
    /// ```
    /// use voml_types::Von;
    /// ```
    #[inline]
    pub fn get_list(&self) -> Option<&Integer> {
        match self {
            Von::Integer(v) => Some(&**v),
            _ => None,
        }
    }
    /// Get mutable reference if the value is dict
    ///
    ///
    /// # Arguments
    ///
    /// * `a`:
    ///
    /// returns: Option<&mut Dict<Von>>
    ///
    /// # Examples
    ///
    /// ```
    /// use voml_types::Von;
    /// ```
    #[inline]
    pub fn mut_list(&mut self) -> Option<&mut Integer> {
        match self {
            Von::Integer(v) => Some(&mut **v),
            _ => None,
        }
    }
    /// Get mutable reference if the value is dict
    ///
    ///
    /// # Arguments
    ///
    /// * `a`:
    ///
    /// returns: Option<&mut Dict<Von>>
    ///
    /// # Examples
    ///
    /// ```
    /// use voml_types::Von;
    /// ```
    #[inline]
    pub fn get_dict(&self) -> Option<&Dict<Von>> {
        match self {
            Von::Dict(v) => Some(&**v),
            _ => None,
        }
    }
    /// Get mutable reference if the value is dict
    ///
    ///
    /// # Arguments
    ///
    /// * `a`:
    ///
    /// returns: Option<&mut Dict<Von>>
    ///
    /// # Examples
    ///
    /// ```
    /// use voml_types::Von;
    /// ```
    #[inline]
    pub fn mut_dict(&mut self) -> Option<&mut Dict<Von>> {
        match self {
            Von::Dict(v) => Some(&mut **v),
            _ => None,
        }
    }
    // pub fn get_integer(&self) {
    //     match self {
    //         Von::Integer(v) => => Some(& **v),
    //         _ => None,
    //     }
    // }
    // pub fn mut_integer(&mut self) {
    //     match self {
    //         Von::Integer(v) => Some(&mut **v),
    //         _ => None,
    //     }
    // }
}
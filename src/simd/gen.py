def joined(n, joiner, f):
    return joiner.join(map(f, range(n)))


def b32(
    n,
    impl, rep,
    vneg_i32,
    vand, vor, vnot,
    align = None,
    load  = None,
    store = None,
):
    name = f"B32x{n}"

    load  = load  or (lambda this: f"{this}.v")
    store = store or (lambda v: v)

    v_decls = joined(n, ", ", lambda i: f"v{i}: bool")
    v_vars  = joined(n, ", ", lambda i: f"v{i}")
    vs_vars   = joined(n, ", ", lambda i: f"vs[{i}]")

    neg_vars = joined(n, "\n        ", lambda i: f"let v{i} = -(v{i} as i32);")

    u32s_as_u8s = joined(n, ", ", lambda i: f"u32s[{i}] as u8")

    return f"""\
#[derive(Clone, Copy)]
#[repr({rep})]
pub struct {name} {{
    v: {impl},
}}

impl {name} {{
    #[inline(always)]
    pub fn new({v_decls}) -> Self {{
        {neg_vars}
        unsafe {{ transmute([{v_vars}]) }}
    }}

    #[inline(always)]
    pub fn from_array(vs: [bool; {n}]) -> Self {{
        Self::new({vs_vars})
    }}

    #[inline(always)]
    pub fn to_array_u32_01(self) -> [u32; {n}] {{
        unsafe {{ transmute({vneg_i32}(transmute(self.v))) }}
    }}

    #[inline(always)]
    pub fn to_array(self) -> [bool; {n}] {{
        let u32s = self.to_array_u32_01();
        unsafe {{ transmute([{u32s_as_u8s}]) }}
    }}
}}


impl core::fmt::Debug for B32x{n} {{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {{
        self.to_array_u32_01().fmt(f)
    }}
}}


impl {name} {{
    #[inline(always)]
    pub fn as_u32(self) -> U32x{n} {{ unsafe {{ transmute(self) }} }}

    #[inline(always)]
    pub fn as_i32(self) -> U32x{n} {{ unsafe {{ transmute(self) }} }}
}}


impl B32x{n} {{
    #[inline(always)]
    pub fn select_u32(self, on_false: U32x{n}, on_true: U32x{n}) -> U32x{n} {{ unsafe {{
        let this     = {load("self")};
        let on_false = {load("on_false")};
        let on_true  = {load("on_true")};
        let r = {vor}(
            {vand}({vnot}(this), on_false),
            {vand}(this, on_true));
        U32x{n} {{ v: {store("r")} }}
    }}}}

    #[inline(always)]
    pub fn select_b32(self, on_false: B32x{n}, on_true: B32x{n}) -> B32x{n} {{
        unsafe {{ transmute(self.select_u32(transmute(on_false), transmute(on_true))) }}
    }}

    #[inline(always)]
    pub fn select_i32(self, on_false: I32x{n}, on_true: I32x{n}) -> I32x{n} {{
        unsafe {{ transmute(self.select_u32(transmute(on_false), transmute(on_true))) }}
    }}

    #[inline(always)]
    pub fn select_f32(self, on_false: F32x{n}, on_true: F32x{n}) -> F32x{n} {{
        unsafe {{ transmute(self.select_u32(transmute(on_false), transmute(on_true))) }}
    }}
}}


impl core::ops::Not for B32x{n} {{
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {{ unsafe {{
        let r = {vnot}({load("self")});
        Self {{ v: {store("r")} }}
    }}}}
}}

impl core::ops::BitAnd for B32x{n} {{
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {{ unsafe {{
        let r = {vand}({load("self")}, {load("rhs")});
        Self {{ v: {store("r")} }}
    }}}}
}}

impl core::ops::BitOr for B32x{n} {{
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {{ unsafe {{
        let r = {vor}({load("self")}, {load("rhs")});
        Self {{ v: {store("r")} }}
    }}}}
}}


"""



def basics(n, name, rep, impl, ty):
    v_decls = joined(n, ", ", lambda i: f"v{i}: {ty}")
    v_vars  = joined(n, ", ", lambda i: f"v{i}")

    return f"""\
#[derive(Clone, Copy)]
#[repr({rep})]
pub struct {name} {{
    v: {impl},
}}

impl {name} {{
    #[inline(always)]
    pub fn new({v_decls}) -> Self {{
        Self::from_array([{v_vars}])
    }}

    #[inline(always)]
    pub fn from_array(vs: [{ty}; {n}]) -> Self {{
        unsafe {{ transmute(vs) }}
    }}

    #[inline(always)]
    pub fn to_array(self) -> [{ty}; {n}] {{
        unsafe {{ transmute(self.v) }}
    }}
}}


impl core::fmt::Debug for {name} {{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {{
        self.to_array().fmt(f)
    }}
}}
"""



def arithmetic(
    name,
    vadd, vsub,
    load, store,
):
    return f"""\
impl core::ops::Add for {name} {{
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {{ unsafe {{
        let r = {vadd}({load("self")}, {load("rhs")});
        Self {{ v: {store("r")} }}
    }}}}
}}

impl core::ops::Sub for {name} {{
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {{ unsafe {{
        let r = {vsub}({load("self")}, {load("rhs")});
        Self {{ v: {store("r")} }}
    }}}}
}}
"""



def comparisons(
    n, name,
    veq, vne, vle, vlt, vge, vgt,
    load, store
):
    if vne:
        ne_impl = f"""
    #[inline(always)]
    pub fn ne(self, other: Self) -> B32x{n} {{ unsafe {{
        let r = {vne}({load("self")}, {load("other")});
        B32x{n} {{ v: {store("r")} }}
    }}}}
"""
    else:
        ne_impl = f"""
    #[inline(always)]
    pub fn ne(self, other: Self) -> B32x{n} {{
        !self.eq(other)
    }}
"""

    return f"""\
impl {name} {{
    #[inline(always)]
    pub fn eq(self, other: Self) -> B32x{n} {{ unsafe {{
        let r = {veq}({load("self")}, {load("other")});
        B32x{n} {{ v: {store("r")} }}
    }}}}

    {ne_impl}

    #[inline(always)]
    pub fn le(self, other: Self) -> B32x{n} {{ unsafe {{
        let r = {vle}({load("self")}, {load("other")});
        B32x{n} {{ v: {store("r")} }}
    }}}}

    #[inline(always)]
    pub fn lt(self, other: Self) -> B32x{n} {{ unsafe {{
        let r = {vlt}({load("self")}, {load("other")});
        B32x{n} {{ v: {store("r")} }}
    }}}}

    #[inline(always)]
    pub fn ge(self, other: Self) -> B32x{n} {{ unsafe {{
        let r = {vge}({load("self")}, {load("other")});
        B32x{n} {{ v: {store("r")} }}
    }}}}

    #[inline(always)]
    pub fn gt(self, other: Self) -> B32x{n} {{ unsafe {{
        let r = {vgt}({load("self")}, {load("other")});
        B32x{n} {{ v: {store("r")} }}
    }}}}
}}
"""



def i32(
    n,
    impl, rep,
    vadd, vsub,
    veq, vne, vle, vlt, vge, vgt,
    align = None,
    load  = None,
    store = None,
):
    name = f"I32x{n}"

    load  = load  or (lambda this: f"{this}.v")
    store = store or (lambda v: v)

    return f"""\
{basics(n, name, rep, impl, "i32")}

impl {name} {{
    #[inline(always)]
    pub fn as_u32(self) -> U32x{n} {{ unsafe {{ transmute(self) }} }}
}}


{arithmetic(name, vadd, vsub, load, store)}

{comparisons(n, name, veq, vne, vle, vlt, vge, vgt, load, store)}

"""



def u32(
    n,
    impl, rep,
    vadd, vsub,
    veq, vne, vle, vlt, vge, vgt,
    align = None,
    load  = None,
    store = None,
):
    name = f"U32x{n}"

    load  = load  or (lambda this: f"{this}.v")
    store = store or (lambda v: v)

    return f"""\
{basics(n, name, rep, impl, "u32")}

impl {name} {{
    #[inline(always)]
    pub fn as_i32(self) -> I32x{n} {{ unsafe {{ transmute(self) }} }}
}}


{arithmetic(name, vadd, vsub, load, store)}

{comparisons(n, name, veq, vne, vle, vlt, vge, vgt, load, store)}

"""




def f32(
    n,
    impl, rep,
    vadd, vsub, vmul, vdiv,
    veq, vne, vle, vlt, vge, vgt,
    align = None,
    load  = None,
    store = None,
):
    name = f"F32x{n}"

    load  = load  or (lambda this: f"{this}.v")
    store = store or (lambda v: v)

    return f"""\
{basics(n, name, rep, impl, "f32")}

impl {name} {{
    #[inline(always)]
    pub fn to_bits(self) -> U32x{n} {{ unsafe {{ transmute(self) }} }}

    #[inline(always)]
    pub fn from_bits(v: U32x{n}) -> Self {{ unsafe {{ transmute(v) }} }}
}}


{arithmetic(name, vadd, vsub, load, store)}\

impl core::ops::Mul for {name} {{
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {{ unsafe {{
        let r = {vmul}({load("self")}, {load("rhs")});
        Self {{ v: {store("r")} }}
    }}}}
}}

impl core::ops::Div for {name} {{
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {{ unsafe {{
        let r = {vdiv}({load("self")}, {load("rhs")});
        Self {{ v: {store("r")} }}
    }}}}
}}


{comparisons(n, name, veq, vne, vle, vlt, vge, vgt, load, store)}

"""



def aarch64():
    r = ""

    r += """\
use core::arch::aarch64::*;
use core::mem::transmute;\n\n"""

    # b32
    r += b32(
        n = 2,
        impl = "uint32x2_t", rep = "align(8)",
        vneg_i32 = "vneg_s32",
        vand = "vand_u32", vor = "vorr_u32", vnot = "vmvn_u32",
    )
    r += b32(
        n = 4,
        impl = "uint32x4_t", rep = "align(16)",
        vneg_i32 = "vnegq_s32",
        vand = "vandq_u32", vor = "vorrq_u32", vnot = "vmvnq_u32",
    )

    # i32
    r += i32(
        n = 2,
        impl = "int32x2_t", rep = "align(8)",
        vadd = "vadd_s32", vsub = "vsub_s32",
        veq  = "vceq_s32", vne  = None,
        vle  = "vcle_s32", vlt  = "vclt_s32",
        vge  = "vcge_s32", vgt  = "vcgt_s32",
    )
    r += i32(
        n = 4,
        impl = "int32x4_t", rep = "align(16)",
        vadd = "vaddq_s32", vsub = "vsubq_s32",
        veq  = "vceqq_s32", vne  = None,
        vle  = "vcleq_s32", vlt  = "vcltq_s32",
        vge  = "vcgeq_s32", vgt  = "vcgtq_s32",
    )

    # u32
    r += u32(
        n = 2,
        impl = "uint32x2_t", rep = "align(8)",
        vadd = "vadd_u32", vsub = "vsub_u32",
        veq  = "vceq_u32", vne  = None,
        vle  = "vcle_u32", vlt  = "vclt_u32",
        vge  = "vcge_u32", vgt  = "vcgt_u32",
    )
    r += u32(
        n = 4,
        impl = "uint32x4_t", rep = "align(16)",
        vadd = "vaddq_u32", vsub = "vsubq_u32",
        veq  = "vceqq_u32", vne  = None,
        vle  = "vcleq_u32", vlt  = "vcltq_u32",
        vge  = "vcgeq_u32", vgt  = "vcgtq_u32",
    )

    # f32
    r += f32(
        n = 2,
        impl = "float32x2_t", rep = "align(8)",
        vadd = "vadd_f32", vsub = "vsub_f32",
        vmul = "vmul_f32", vdiv = "vdiv_f32",
        veq  = "vceq_f32", vne  = None,
        vle  = "vcle_f32", vlt  = "vclt_f32",
        vge  = "vcge_f32", vgt  = "vcgt_f32",
    )
    r += f32(
        n = 4,
        impl = "float32x4_t", rep = "align(16)",
        vadd = "vaddq_f32", vsub = "vsubq_f32",
        vmul = "vmulq_f32", vdiv = "vdivq_f32",
        veq  = "vceqq_f32", vne  = None,
        vle  = "vcleq_f32", vlt  = "vcltq_f32",
        vge  = "vcgeq_f32", vgt  = "vcgtq_f32",
    )

    return r



print(aarch64())



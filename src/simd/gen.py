def joined(n, joiner, f):
    return joiner.join(map(f, range(n)))


def b32(
    n,
    impl, rep,
    vand, vor, vnot,
    vselect = None,
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

    if vselect:
        select_impl = vselect
    else:
        select_impl = f"""\
        {vor}(
            {vand}({vnot}(this), on_false),
            {vand}(this, on_true))"""

    return f"""\
#[derive(Clone, Copy)]
#[repr({rep})]
pub struct {name} {{
    v: {impl},
}}

impl {name} {{
    pub const NONE: {name} = {name}::splat(false);
    pub const ALL:  {name} = {name}::splat(true);

    #[inline(always)]
    pub const fn new({v_decls}) -> Self {{
        {neg_vars}
        unsafe {{ transmute([{v_vars}]) }}
    }}

    #[inline(always)]
    pub const fn splat(v: bool) -> Self {{
        Self::from_array([v; {n}])
    }}

    #[inline(always)]
    pub const fn from_array(vs: [bool; {n}]) -> Self {{
        Self::new({vs_vars})
    }}

    #[inline(always)]
    pub fn to_array_u32_01(self) -> [u32; {n}] {{
        (-self.as_u32()).to_array()
    }}

    #[inline(always)]
    pub fn to_array(self) -> [bool; {n}] {{
        let u32s = self.to_array_u32_01();
        unsafe {{ transmute([{u32s_as_u8s}]) }}
    }}
}}


impl Into<B32x{n}> for bool {{
    #[inline(always)]
    fn into(self) -> B32x{n} {{
        B32x{n}::splat(self)
    }}
}}

impl Into<B32x{n}> for [bool; {n}] {{
    #[inline(always)]
    fn into(self) -> B32x{n} {{
        B32x{n}::from_array(self)
    }}
}}

impl Default for B32x{n} {{
    #[inline(always)]
    fn default() -> Self {{ Self::NONE }}
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
        let r = {select_impl};
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

    #[inline(always)]
    pub fn any(self) -> bool {{
        self.as_u32().hmax() != 0
    }}

    #[inline(always)]
    pub fn all(self) -> bool {{
        (!self).as_u32().hmax() == 0
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



def basics(n, name, rep, impl, ty, kzero, kone, kmin, kmax):
    v_decls = joined(n, ", ", lambda i: f"v{i}: {ty}")
    v_vars  = joined(n, ", ", lambda i: f"v{i}")

    return f"""\
#[derive(Clone, Copy)]
#[repr({rep})]
pub struct {name} {{
    v: {impl},
}}

impl {name} {{
    pub const ZERO: {name} = {name}::splat({kzero});
    pub const ONE:  {name} = {name}::splat({kone});
    pub const MIN:  {name} = {name}::splat({kmin});
    pub const MAX:  {name} = {name}::splat({kmax});

    #[inline(always)]
    pub const fn new({v_decls}) -> Self {{
        Self::from_array([{v_vars}])
    }}

    #[inline(always)]
    pub const fn splat(v: {ty}) -> Self {{
        Self::from_array([v; {n}])
    }}

    #[inline(always)]
    pub const fn from_array(vs: [{ty}; {n}]) -> Self {{
        unsafe {{ transmute(vs) }}
    }}

    #[inline(always)]
    pub const fn to_array(self) -> [{ty}; {n}] {{
        unsafe {{ transmute(self.v) }}
    }}
}}

impl Into<{name}> for {ty} {{
    #[inline(always)]
    fn into(self) -> {name} {{
        {name}::splat(self)
    }}
}}

impl Into<{name}> for [{ty}; {n}] {{
    #[inline(always)]
    fn into(self) -> {name} {{
        {name}::from_array(self)
    }}
}}

impl Default for {name} {{
    #[inline(always)]
    fn default() -> Self {{ Self::ZERO }}
}}


impl core::fmt::Debug for {name} {{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {{
        self.to_array().fmt(f)
    }}
}}
"""



def elements(
    name, ty, n,
):
    assert n >= 2

    xyzw = ""
    if n == 2 or n == 4:
        zw = ""
        if n == 4:
            zw = f"""

    #[inline(always)]
    pub fn z(self) -> {ty} {{ self[2] }}

    #[inline(always)]
    pub fn w(self) -> {ty} {{ self[3] }}\
"""

        xyzw = f"""\
impl {name} {{
    #[inline(always)]
    pub fn x(self) -> {ty} {{ self[0] }}

    #[inline(always)]
    pub fn y(self) -> {ty} {{ self[1] }}\
{zw}
}}"""

    return f"""\
{xyzw}

impl core::ops::Deref for {name} {{
    type Target = [{ty}; {n}];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {{
        unsafe {{ transmute(&self.v) }}
    }}
}}

impl core::ops::DerefMut for {name} {{
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {{
        unsafe {{ transmute(&mut self.v) }}
    }}
}}
"""



def arithmetic(
    name,
    vadd, vsub,
    vneg,
    load, store,
):
    if type(vneg) is tuple:
        neg_impl = vneg[0]
    else:
        neg_impl = f"""\
    unsafe {{
        let r = {vneg}({load("self")});
        Self {{ v: {store("r")} }}
    }}"""

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

impl core::ops::Neg for {name} {{
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {{
        {neg_impl}
    }}
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

impl PartialEq for {name} {{
    fn eq(&self, other: &Self) -> bool {{
        {name}::eq(*self, *other).all()
    }}
}}
"""


def ord_stuff(
    name, ty,
    vmin, vmax,
    vhmin, vhmax,
    load, store
):
    return f"""\
impl {name} {{
    #[inline(always)]
    pub fn min(self, other: Self) -> Self {{ unsafe {{
        let r = {vmin}({load("self")}, {load("other")});
        Self {{ v: {store("r")} }}
    }}}}

    #[inline(always)]
    pub fn max(self, other: Self) -> Self {{ unsafe {{
        let r = {vmax}({load("self")}, {load("other")});
        Self {{ v: {store("r")} }}
    }}}}

    #[inline(always)]
    pub fn at_least(self, other: Self) -> Self {{
        self.max(other)
    }}

    #[inline(always)]
    pub fn at_most(self, other: Self) -> Self {{
        self.min(other)
    }}

    #[inline(always)]
    pub fn clamp(self, low: Self, high: Self) -> Self {{
        self.at_least(low).at_most(high)
    }}


    #[inline(always)]
    pub fn hmin(self) -> {ty} {{ unsafe {{
        {vhmin}({load("self")})
    }}}}

    #[inline(always)]
    pub fn hmax(self) -> {ty} {{ unsafe {{
        {vhmax}({load("self")})
    }}}}
}}
"""



def i32(
    n,
    impl, rep,
    vadd, vsub,
    veq, vne, vle, vlt, vge, vgt,
    vneg,
    vmin, vmax,
    vhmin, vhmax,
    vcvt_f32,
    align = None,
    load  = None,
    store = None,
):
    name = f"I32x{n}"

    load  = load  or (lambda this: f"{this}.v")
    store = store or (lambda v: v)

    return f"""\
{basics(n, name, rep, impl, "i32", "0", "1", "i32::MIN", "i32::MAX")}

{elements(name, "i32", n)}

impl {name} {{
    #[inline(always)]
    pub fn as_u32(self) -> U32x{n} {{ unsafe {{ transmute(self) }} }}

    #[inline(always)]
    pub fn to_f32(self) -> F32x{n} {{ unsafe {{
        let r = {vcvt_f32}({load("self")});
        F32x{n} {{ v: {store("r")} }}
    }}}}
}}


{arithmetic(name, vadd, vsub, vneg, load, store)}

{ord_stuff(name, "i32", vmin, vmax, vhmin, vhmax, load, store)}

{comparisons(n, name, veq, vne, vle, vlt, vge, vgt, load, store)}

"""



def u32(
    n,
    impl, rep,
    vadd, vsub,
    veq, vne, vle, vlt, vge, vgt,
    vneg,
    vmin, vmax,
    vhmin, vhmax,
    align = None,
    load  = None,
    store = None,
):
    name = f"U32x{n}"

    load  = load  or (lambda this: f"{this}.v")
    store = store or (lambda v: v)

    return f"""\
{basics(n, name, rep, impl, "u32", "0", "1", "u32::MIN", "u32::MAX")}

{elements(name, "u32", n)}

impl {name} {{
    #[inline(always)]
    pub fn as_i32(self) -> I32x{n} {{ unsafe {{ transmute(self) }} }}
}}


{arithmetic(name, vadd, vsub, vneg, load, store)}

{ord_stuff(name, "u32", vmin, vmax, vhmin, vhmax, load, store)}

{comparisons(n, name, veq, vne, vle, vlt, vge, vgt, load, store)}

"""




def f32(
    n,
    impl, rep,
    vadd, vsub, vmul, vdiv,
    veq, vne, vle, vlt, vge, vgt,
    vneg,
    vmin, vmax,
    vhmin, vhmax,
    vcvt_i32,
    vfloor, vceil, vround, vtrunc,
    align = None,
    load  = None,
    store = None,
):
    name = f"F32x{n}"

    load  = load  or (lambda this: f"{this}.v")
    store = store or (lambda v: v)

    return f"""\
{basics(n, name, rep, impl, "f32", "0.0", "1.0", "f32::MIN", "f32::MAX")}

{elements(name, "f32", n)}

impl {name} {{
    /// behavior for values outside the `i32` range is platform dependent
    /// and considered a bug (there is no guarantee that the program won't crash).
    /// technically, this function should be unsafe, but that would make it rather
    /// annoying to use.
    #[inline(always)]
    pub fn to_i32_unck(self) -> I32x{n} {{ unsafe {{
        let r = {vcvt_i32}({load("self")});
        I32x{n} {{ v: {store("r")} }}
    }}}}
}}

impl {name} {{
    #[inline(always)]
    pub const fn to_bits(self) -> U32x{n} {{ unsafe {{ transmute(self) }} }}

    #[inline(always)]
    pub const fn from_bits(v: U32x{n}) -> Self {{ unsafe {{ transmute(v) }} }}

    #[inline(always)]
    pub fn floor(self) -> Self {{ unsafe {{
        let r = {vfloor}({load("self")});
        Self {{ v: {store("r")} }}
    }}}}

    #[inline(always)]
    pub fn ceil(self) -> Self {{ unsafe {{
        let r = {vceil}({load("self")});
        Self {{ v: {store("r")} }}
    }}}}

    #[inline(always)]
    pub fn round(self) -> Self {{ unsafe {{
        let r = {vround}({load("self")});
        Self {{ v: {store("r")} }}
    }}}}

    #[inline(always)]
    pub fn trunc(self) -> Self {{ unsafe {{
        let r = {vtrunc}({load("self")});
        Self {{ v: {store("r")} }}
    }}}}

    #[inline(always)]
    pub fn lerp(self, other: Self, t: f32) -> Self {{
        self.lerpv(other, t.into())
    }}

    #[inline(always)]
    pub fn lerpv(self, other: Self, ts: Self) -> Self {{
        (Self::ONE - ts)*self + ts*other
    }}
}}


{arithmetic(name, vadd, vsub, vneg, load, store)}\

{ord_stuff(name, "f32", vmin, vmax, vhmin, vhmax, load, store)}

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
        vand = "vand_u32", vor = "vorr_u32", vnot = "vmvn_u32",
        vselect = "vbsl_u32(this, on_true, on_false)",
    )
    r += b32(
        n = 4,
        impl = "uint32x4_t", rep = "align(16)",
        vand = "vandq_u32", vor = "vorrq_u32", vnot = "vmvnq_u32",
        vselect = "vbslq_u32(this, on_true, on_false)",
    )

    # i32
    r += i32(
        n = 2,
        impl = "int32x2_t", rep = "align(8)",
        vadd = "vadd_s32", vsub = "vsub_s32",
        veq  = "vceq_s32", vne  = None,
        vle  = "vcle_s32", vlt  = "vclt_s32",
        vge  = "vcge_s32", vgt  = "vcgt_s32",
        vneg = "vneg_s32",
        vmin = "vmin_s32", vmax = "vmax_s32",
        vhmin = "vminv_s32", vhmax = "vmaxv_s32",
        vcvt_f32 = "vcvt_f32_s32",
    )
    r += i32(
        n = 4,
        impl = "int32x4_t", rep = "align(16)",
        vadd = "vaddq_s32", vsub = "vsubq_s32",
        veq  = "vceqq_s32", vne  = None,
        vle  = "vcleq_s32", vlt  = "vcltq_s32",
        vge  = "vcgeq_s32", vgt  = "vcgtq_s32",
        vneg = "vnegq_s32",
        vmin = "vminq_s32", vmax = "vmaxq_s32",
        vhmin = "vminvq_s32", vhmax = "vmaxvq_s32",
        vcvt_f32 = "vcvtq_f32_s32",
    )

    # u32
    r += u32(
        n = 2,
        impl = "uint32x2_t", rep = "align(8)",
        vadd = "vadd_u32", vsub = "vsub_u32",
        veq  = "vceq_u32", vne  = None,
        vle  = "vcle_u32", vlt  = "vclt_u32",
        vge  = "vcge_u32", vgt  = "vcgt_u32",
        vmin = "vmin_u32", vmax = "vmax_u32",
        vhmin = "vminv_u32", vhmax = "vmaxv_u32",
        vneg = ("(-self.as_i32()).as_u32()",),
    )
    r += u32(
        n = 4,
        impl = "uint32x4_t", rep = "align(16)",
        vadd = "vaddq_u32", vsub = "vsubq_u32",
        veq  = "vceqq_u32", vne  = None,
        vle  = "vcleq_u32", vlt  = "vcltq_u32",
        vge  = "vcgeq_u32", vgt  = "vcgtq_u32",
        vmin = "vminq_u32", vmax = "vmaxq_u32",
        vhmin = "vminvq_u32", vhmax = "vmaxvq_u32",
        vneg = ("(-self.as_i32()).as_u32()",),
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
        vneg = "vneg_f32",
        vmin = "vmin_f32", vmax = "vmax_f32",
        vhmin = "vminv_f32", vhmax = "vmaxv_f32",
        vfloor = "vrndm_f32", vceil = "vrndp_f32",
        vround = "vrndn_f32", vtrunc = "vrnd_f32",
        vcvt_i32 = "vcvtm_s32_f32",
    )
    r += f32(
        n = 4,
        impl = "float32x4_t", rep = "align(16)",
        vadd = "vaddq_f32", vsub = "vsubq_f32",
        vmul = "vmulq_f32", vdiv = "vdivq_f32",
        veq  = "vceqq_f32", vne  = None,
        vle  = "vcleq_f32", vlt  = "vcltq_f32",
        vge  = "vcgeq_f32", vgt  = "vcgtq_f32",
        vneg = "vnegq_f32",
        vmin = "vminq_f32", vmax = "vmaxq_f32",
        vhmin = "vminvq_f32", vhmax = "vmaxvq_f32",
        vfloor = "vrndmq_f32", vceil = "vrndpq_f32",
        vround = "vrndnq_f32", vtrunc = "vrndq_f32",
        vcvt_i32 = "vcvtmq_s32_f32",
    )

    return r



print(aarch64())



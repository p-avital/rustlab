use core::ops::*;

pub trait Signed {
    type Add<Rhs: Signed>: Signed;
    type Sub<Rhs: Signed>: Signed;
    type Mul<Rhs: Signed>: Signed;
}

pub struct Noted<Value, Units> {
    value: Value,
    units: Units,
}

pub struct ISU<
    Sec: Signed,
    Meter: Signed,
    Kg: Signed,
    Amp: Signed,
    Kel: Signed,
    Mol: Signed,
    Cd: Signed,
>(core::marker::PhantomData<(Sec, Meter, Kg, Amp, Kel, Mol, Cd)>);
impl<Sec: Signed, Meter: Signed, Kg: Signed, Amp: Signed, Kel: Signed, Mol: Signed, Cd: Signed> Add
    for ISU<Sec, Meter, Kg, Amp, Kel, Mol, Cd>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self
    }
}
impl<Sec: Signed, Meter: Signed, Kg: Signed, Amp: Signed, Kel: Signed, Mol: Signed, Cd: Signed> Sub
    for ISU<Sec, Meter, Kg, Amp, Kel, Mol, Cd>
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self
    }
}
impl<
        Sec: Signed,
        Meter: Signed,
        Kg: Signed,
        Amp: Signed,
        Kel: Signed,
        Mol: Signed,
        Cd: Signed,
        Sec2: Signed,
        Meter2: Signed,
        Kg2: Signed,
        Amp2: Signed,
        Kel2: Signed,
        Mol2: Signed,
        Cd2: Signed,
    > Mul<ISU<Sec2, Meter2, Kg2, Amp2, Kel2, Mol2, Cd2>>
    for ISU<Sec, Meter, Kg, Amp, Kel, Mol, Cd>
{
    type Output = ISU<
        Sec::Add<Sec2>,
        Meter::Add<Meter2>,
        Kg::Add<Kg2>,
        Amp::Add<Amp2>,
        Kel::Add<Kel2>,
        Mol::Add<Mol2>,
        Cd::Add<Cd2>,
    >;
    fn mul(self, rhs: ISU<Sec2, Meter2, Kg2, Amp2, Kel2, Mol2, Cd2>) -> Self::Output {
        ISU(core::marker::PhantomData)
    }
}
impl<
        Sec: Signed,
        Meter: Signed,
        Kg: Signed,
        Amp: Signed,
        Kel: Signed,
        Mol: Signed,
        Cd: Signed,
        Sec2: Signed,
        Meter2: Signed,
        Kg2: Signed,
        Amp2: Signed,
        Kel2: Signed,
        Mol2: Signed,
        Cd2: Signed,
    > Div<ISU<Sec2, Meter2, Kg2, Amp2, Kel2, Mol2, Cd2>>
    for ISU<Sec, Meter, Kg, Amp, Kel, Mol, Cd>
{
    type Output = ISU<
        Sec::Sub<Sec2>,
        Meter::Sub<Meter2>,
        Kg::Sub<Kg2>,
        Amp::Sub<Amp2>,
        Kel::Sub<Kel2>,
        Mol::Sub<Mol2>,
        Cd::Sub<Cd2>,
    >;
    fn div(self, rhs: ISU<Sec2, Meter2, Kg2, Amp2, Kel2, Mol2, Cd2>) -> Self::Output {
        ISU(core::marker::PhantomData)
    }
}

impl<V1, V2, U1, U2> core::ops::Add<Noted<V1, U1>> for Noted<V2, U2>
where
    V2: Add<V1>,
    U2: Add<U1>,
{
    type Output = Noted<<V2 as core::ops::Add<V1>>::Output, <U2 as core::ops::Add<U1>>::Output>;
    fn add(self, rhs: Noted<V1, U1>) -> Self::Output {
        Noted {
            value: self.value + rhs.value,
            units: self.units + rhs.units,
        }
    }
}

impl<V1, V2, U1, U2> core::ops::Mul<Noted<V1, U1>> for Noted<V2, U2>
where
    V2: Mul<V1>,
    U2: Mul<U1>,
{
    type Output = Noted<<V2 as core::ops::Mul<V1>>::Output, <U2 as core::ops::Mul<U1>>::Output>;
    fn mul(self, rhs: Noted<V1, U1>) -> Self::Output {
        Noted {
            value: self.value * rhs.value,
            units: self.units * rhs.units,
        }
    }
}

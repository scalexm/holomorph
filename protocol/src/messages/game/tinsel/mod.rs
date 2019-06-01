use protocol_derive::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6368)]
pub struct OrnamentGainedMessage<'a> {
    pub ornament_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6364)]
pub struct TitleGainedMessage<'a> {
    #[protocol(var)]
    pub title_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6370)]
pub struct OrnamentSelectErrorMessage<'a> {
    pub reason: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6373)]
pub struct TitleSelectErrorMessage<'a> {
    pub reason: u8,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6369)]
pub struct OrnamentSelectedMessage<'a> {
    #[protocol(var)]
    pub ornament_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6371)]
pub struct TitleLostMessage<'a> {
    #[protocol(var)]
    pub title_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6366)]
pub struct TitleSelectedMessage<'a> {
    #[protocol(var)]
    pub title_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6363)]
pub struct TitlesAndOrnamentsListRequestMessage<'a> {
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6367)]
pub struct TitlesAndOrnamentsListMessage<'a> {
    #[protocol(var_contents)]
    pub titles: std::borrow::Cow<'a, [u16]>,
    #[protocol(var_contents)]
    pub ornaments: std::borrow::Cow<'a, [u16]>,
    #[protocol(var)]
    pub active_title: u16,
    #[protocol(var)]
    pub active_ornament: u16,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6365)]
pub struct TitleSelectRequestMessage<'a> {
    #[protocol(var)]
    pub title_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6374)]
pub struct OrnamentSelectRequestMessage<'a> {
    #[protocol(var)]
    pub ornament_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
#[protocol(id = 6770)]
pub struct OrnamentLostMessage<'a> {
    pub ornament_id: u16,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

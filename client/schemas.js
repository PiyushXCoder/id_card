const cardDataSchema = {
  enum: [
    {
      struct: {
        CreateCard: { struct: { name: "string", bio: "string", bump: "u8" } },
      },
    },
    {
      struct: {
        UpdateCard: { struct: { id: "u64", name: "string", bio: "string" } },
      },
    },
    {
      struct: {
        DeleteCard: { struct: { id: "u64" } },
      },
    },
  ],
};

export { cardDataSchema };

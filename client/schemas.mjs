const cardDataSchema = {
  enum: [
    {
      struct: {
        CreateCard: { struct: { id: "u64", name: "string", bio: "string" } },
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

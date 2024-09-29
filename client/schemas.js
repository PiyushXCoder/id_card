const cardDataSchema = {
  enum: [
    {
      struct: {
        Create: { struct: { name: "string", bio: "string", bump: "u8" } },
      },
    },
    {
      struct: {
        Update: { struct: { name: "string", bio: "string" } },
      },
    },
    {
      struct: {
        Delete: { struct: { id: "u64" } },
      },
    },
  ],
};

export { cardDataSchema };

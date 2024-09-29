const cardDataSchema = {
  enum: [
    {
      struct: {
        Create: { struct: { name: "string", bio: "string", bump: "u8" } },
      },
    },
    {
      struct: {
        Update: {
          struct: { name: { option: "string" }, bio: { option: "string" } },
        },
      },
    },
    {
      struct: {
        Delete: { struct: { id: "u64" } },
      },
    },
  ],
};

const cardSchema = {
  struct: { name: "string", bio: "string", bump: "u8" },
};

export { cardDataSchema, cardSchema };

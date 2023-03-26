module.exports = {
  darkMode: 'class',
  content: ["./src/**/*.rs", "./static/index.html"],
  theme: {
    extend: {
      textColor: {
        primary: "var(--color-text-primary)"
      },
      backgroundColor: {
        primary: "var(--color-bg-primary)",
      }
    },
  },
  plugins: [],
};

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
      },
      container: {
        // you can configure the container to be centered
        center: true,

        // or have default horizontal padding
        padding: '1rem',

        // default breakpoints but with 40px removed
        screens: {
          sm: '600px',
          md: '728px',
          lg: '984px',
          xl: '1100px',
          '2xl': '1100px',
        },
      },
    },
  },
  plugins: [],
};

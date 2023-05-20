import type { Config } from "tailwindcss";

export default {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {},
  },
  plugins: [require("tailwindcss-animate")],
} satisfies Config;

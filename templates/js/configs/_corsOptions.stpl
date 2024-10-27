import { allowedOrigins } from "./index.js";

const corsOptions = {
    origin: (origin, callback) => {
        if (!origin || allowedOrigins.indexOf(origin) !== -1) {
            callback(null, true)
        } else {
            callback(new Error('Not Allowed by CORS'))
        }
    },
    optionsSuccessStatus: 200,
    credentials: true,
};

export default corsOptions
export { corsOptions }
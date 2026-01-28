import { Provider } from 'react-redux';
import { createStoreWithoutState } from '@theme/ApiItem/store';

// Create store using the plugin's official store factory
const store = createStoreWithoutState({}, []);

export default function Root({ children }) {
    return <Provider store={store}>{children}</Provider>;
}

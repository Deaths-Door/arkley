import { StyleSheet, Platform , StatusBar} from 'react-native';
import AppNavigator from './src/components/AppNavigator';
import { SafeAreaView } from "react-native-safe-area-context";

export default function App() {
   return <SafeAreaView 
      style={{  flex: 1, paddingTop: Platform.OS === 'android' ? StatusBar.currentHeight : 0 }}>
         <AppNavigator/>
   </SafeAreaView>
}

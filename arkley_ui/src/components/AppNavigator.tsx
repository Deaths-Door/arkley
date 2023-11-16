import { NavigationContainer } from "@react-navigation/native";
import { createMaterialTopTabNavigator } from '@react-navigation/material-top-tabs';
import { Icon } from "react-native-paper";

const Tab = createMaterialTopTabNavigator();

export default function AppNavigator() {
    return (
        <NavigationContainer>
            <Tab.Navigator>{
                Screens.map((item) => <Tab.Screen name={item.name} component={item.component}
                options={{ tabBarIcon : () => <Icon source={item.icon} size={20} /> }}/>)    
            }</Tab.Navigator>
        </NavigationContainer>
    )
}

const Screens = [
    { name : "haupt-calc" , component : () => <Icon source="camera" size={20} />  , icon : "camera" },
    { name : "graphing-calc" , component : () => <Icon source="camera" size={20} />  , icon : "camera" },
    { name : "conversion-calc" , component : () => <Icon source="camera" size={20} />  , icon : "camera" },
]
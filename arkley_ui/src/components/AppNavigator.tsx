import { NavigationContainer } from "@react-navigation/native";
import { createMaterialTopTabNavigator } from '@react-navigation/material-top-tabs';
import { Icon } from "react-native-paper";
import ConversionCalculator from "../screens/ConversionCalculator";
import Calculator from "../screens/Calculator";

const Tab = createMaterialTopTabNavigator();

export default function AppNavigator() {
    return (
        <NavigationContainer>
            <Tab.Navigator screenOptions={{ tabBarShowLabel : false }}>{
                Screens.map((item) => <Tab.Screen name={item.name} component={item.component} options={{ tabBarIcon : () => <Icon source={item.icon} size={20} /> }}/>)    
            }</Tab.Navigator>
        </NavigationContainer>
    )
}

const Screens = [
    { name : "haupt-calc" , component : Calculator  , icon : "camera" },
    { name : "conversion-calc" , component : ConversionCalculator  , icon : "camera" },
    { name : "graphing-calc" , component : () => <Icon source="camera" size={20} />  , icon : "camera" },
]
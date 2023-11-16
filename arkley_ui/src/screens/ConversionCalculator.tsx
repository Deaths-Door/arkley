import { View , Dimensions , StyleSheet, TouchableOpacity } from "react-native";
import { Icon , Text } from "react-native-paper";
import { FlatGrid } from 'react-native-super-grid';

export default function ConversionCalculator() {    
    const screenWidth = Dimensions.get("window").width;
    const tileSize = screenWidth / 3 - 20;

    // TODO : impl both methods
    const onItemPress = () => {};
    const onItemLongPress = () => {};

    return (
        <FlatGrid 
            // TODO : Add Filtering + Sort for this but what sort idk
            data={items} 
            itemDimension={tileSize}
            spacing={-10}
            renderItem={({ item }) => {
                const isMultipleWords = item.title.includes(" ");

                const text = isMultipleWords ?  <Text>{item.title}</Text> : 
                    <Text numberOfLines={1} adjustsFontSizeToFit={true} >{item.title}</Text>;

                return (
                    <TouchableOpacity onPress={onItemPress} onLongPress={onItemLongPress}>
                        <View style={styles.item}>
                            <Icon source={item.image} size={50}/>
                            {text}
                        </View>
                    </TouchableOpacity>
                )
            }}
        />
    )
}

const styles = StyleSheet.create({
    item : { alignItems : "center" , padding : 12 }
});

const items = [
    { title: "Age", image: "camera" },
    { title: "Area", image: "camera" },
    { title: "Data", image: "camera" },
    { title: "Date", image: "camera" },
    { title: "Discount", image: "camera" },
    { title: "Length", image: "camera" },
    { title: "Mass", image: "camera" },
    { title: "Numeric System", image: "camera" },
    { title: "Speed", image: "camera" },
    { title: "Temperature", image: "camera" },
    { title: "Time", image: "camera" },
    { title: "Volume", image: "camera" },
    { title: "Volume Flow Rate", image: "camera" },
    { title: "Voltage", image: "camera" },
    { title: "Current", image: "camera" },

    { title: "Currency Conversion", image: "camera" },
    { title: "Energy", image: "camera" },
    { title: "Pressure", image: "camera" },
    { title: "Additional Area", image: "camera" },
    { title: "Additional Volume", image: "camera" },
    { title: "Power", image: "camera" },
    { title: "Additional Length", image: "camera" },
    { title: "Data Storage", image: "camera" },
    { title: "Frequency", image: "camera" },
    { title: "Angle", image: "camera" },
    { title: "Fuel Efficiency", image: "camera" },
    { title: "Density", image: "camera" },
    { title: "Additional Temperature", image: "camera" },
    { title: "Time Units", image: "camera" },
    { title: "Electrical Units", image: "camera" },
    { title: "Percentages", image: "camera" },
    { title: "Cooking Units", image: "camera" },
    { title: "Sound", image: "camera" },
    { title: "Health", image: "camera" },
];
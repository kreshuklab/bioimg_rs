use super::StatefulWidget;

pub enum PreprocessingWidgetMode {
    Binarize,
    Clip,
    ScaleLinear,
    Sigmoid,
    ZeroMeanUnitVariance,
    ScaleRange,
}



pub struct PreprocessingWidget{
    pub mode: PreprocessingWidgetMode,
}


impl StatefulWidget for PreprocessingWidget{
    
}

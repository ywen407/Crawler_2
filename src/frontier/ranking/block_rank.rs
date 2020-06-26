use crate::frontier::coordinator::Coordinator;

impl Coordinator{
     /* implement */
    /*
        블록들의 rank를 계산
        source array, destination array의 모든 원소들의 값 초기화
        source array, destination array의 rank값을 이용하여
        residual를 구하고 이 값이 threshold보다 클 동안 다음의 과정을 반복한다.
        1)먼저 하나의 Link 자료 구조에 기록된 데이터를 읽어온다.
        2)읽어 온 데이터를 이용하여 rank를 계산한다.
        3)계산된 rank를 이용하여 residual값을 계산한다.
        
        residiaul < threshold 일 경우
        source array에 저장괸 결과를 파일에 기록한 뒤 계산을 종료한다.
    */
}


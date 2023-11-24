import { combineReducers, configureStore } from '@reduxjs/toolkit'
import { useDispatch, useSelector } from 'react-redux'

import type { DefaultRootState, TypedUseSelectorHook} from 'react-redux'
import thunk from "redux-thunk";
import UserSlice from './user.slice';

const reducer = combineReducers({
	UserSlice,
})

const store = configureStore({
	reducer,
	middleware: (getDefaultMiddleware) => getDefaultMiddleware().concat(thunk),
})

type StoreType = typeof store

type IAppDispatch = StoreType['dispatch']

export type TRootState = ReturnType<typeof reducer>
declare module 'react-redux' {
	export interface DefaultRootState extends TRootState {}
}
export const useAppDispatch = () => useDispatch<IAppDispatch>()
export const useAppSelector: TypedUseSelectorHook<DefaultRootState> = useSelector
export default store
